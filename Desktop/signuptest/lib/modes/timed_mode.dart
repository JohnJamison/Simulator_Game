import 'dart:async';
import 'dart:math';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../models/question.dart';
import '../data/repo_local.dart';
import '../services/attempt_service.dart';
import 'dart:ui' as ui; // for BackdropFilter blur
//Firebase plugins
import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart';



/* ============================================================================
   SETUP DIALOG (centered modal with snap sliders)
============================================================================ */

enum SectionMode { math, reading, writing, mixed }

class TimedConfig {
  final int totalSeconds; // 300/600/900
  final SectionMode sectionMode;
  const TimedConfig(this.totalSeconds, this.sectionMode);
}

Future<TimedConfig?> showTimedSetup(BuildContext context) {
  final base = Theme.of(context);

  // Apply Helvetica only inside the modal by adjusting the text themes.
  final helvetica = base.copyWith(
    textTheme: base.textTheme.apply(fontFamily: 'Helvetica'),
    primaryTextTheme: base.primaryTextTheme.apply(fontFamily: 'Helvetica'),
  );

  return showGeneralDialog<TimedConfig>(
    context: context,
    barrierDismissible: true,
    barrierLabel: 'Close',
    barrierColor: Colors.black54,
    transitionDuration: const Duration(milliseconds: 180),
    pageBuilder: (_, __, ___) {
      return Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 640, minWidth: 320),
          child: Theme( // <- local font override lives here
            data: helvetica,
            child: Material(
              elevation: 12,
              color: base.colorScheme.surface,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(24)),
              child: const _TimedSetupCard(),
            ),
          ),
        ),
      );
    },
    transitionBuilder: (_, a, __, child) {
      final fade = CurvedAnimation(parent: a, curve: Curves.easeOut);
      final scale = Tween<double>(begin: 0.98, end: 1.0).animate(fade);
      return FadeTransition(opacity: fade, child: ScaleTransition(scale: scale, child: child));
    },
  );
}



class _TimedSetupCard extends StatefulWidget {
  const _TimedSetupCard();
  @override
  State<_TimedSetupCard> createState() => _TimedSetupCardState();
}

class _TimedSetupCardState extends State<_TimedSetupCard> {
  int _sectionIndex = 3; // 0:Math 1:Reading 2:Writing 3:Mixed
  int _timeIndex = 1;    // 0:5 1:10 2:15

  @override
  Widget build(BuildContext context) {
    final options = ['Math', 'Reading', 'Writing', 'Mixed'];
    final times = ['5 min', '10 min', '15 min'];

    return Padding(
      padding: const EdgeInsets.fromLTRB(18, 12, 18, 16),
      child: Column(mainAxisSize: MainAxisSize.min, children: [
        const SizedBox(height: 4),
        Text('Timed Mode Setup',
            style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w800)),
        const SizedBox(height: 16),

        const _DialogLabel('Section'),
        SnapSlider(labels: options, initialIndex: _sectionIndex, onChanged: (i) => setState(() => _sectionIndex = i)),
        const SizedBox(height: 18),

        const _DialogLabel('Time'),
        SnapSlider(labels: times, initialIndex: _timeIndex, onChanged: (i) => setState(() => _timeIndex = i)),
        const SizedBox(height: 16),

        SizedBox(
          width: double.infinity,
          child: FilledButton.icon(
            onPressed: () {
              final seconds = [300, 600, 900][_timeIndex];
              final section = [SectionMode.math, SectionMode.reading, SectionMode.writing, SectionMode.mixed][_sectionIndex];
              Navigator.pop(context, TimedConfig(seconds, section));
            },
            icon: const Icon(Icons.play_arrow),
            label: const Text('Start'),
          ),
        ),
      ]),
    );
  }
}

class _DialogLabel extends StatelessWidget {
  final String text;
  const _DialogLabel(this.text);

  @override
  Widget build(BuildContext context) {
    final t = Theme.of(context).textTheme;
    final c = Theme.of(context).colorScheme;
    return Padding(
      padding: const EdgeInsets.only(bottom: 6),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(text, style: t.titleSmall?.copyWith(fontWeight: FontWeight.w800, color: c.onSurface)),
        ],
      ),
    );
  }
}

/* ============================================================================
   SNAP SLIDER (draggable, snaps to labels, transparent thumb)
============================================================================ */

// ===== THEMEABLE SNAP SLIDER ===============================================

class SnapStyle {
  final double trackHeight;
  final double pillHeight;
  final BorderRadius radius;
  final Gradient? trackGradient;
  final Color trackColor;
  final BoxBorder? trackBorder;

  final Gradient? pillGradient;
  final Color pillColor;               // used if pillGradient == null
  final Color pillBorderColor;
  final double pillBorderWidth;
  final List<BoxShadow> pillShadows;

  final Color labelColor;
  final Color selectedLabelColor;
  final FontWeight selectedWeight;

  const SnapStyle({
    required this.trackHeight,
    required this.pillHeight,
    required this.radius,
    this.trackGradient,
    required this.trackColor,
    this.trackBorder,
    this.pillGradient,
    required this.pillColor,
    required this.pillBorderColor,
    required this.pillBorderWidth,
    required this.pillShadows,
    required this.labelColor,
    required this.selectedLabelColor,
    required this.selectedWeight,
  });
}

class SnapStyles {
  // Thin, modern, neutral (good default)
  static SnapStyle minimal(BuildContext ctx) {
    final cs = Theme.of(ctx).colorScheme;
    return SnapStyle(
      trackHeight: 32,
      pillHeight: 22,
      radius: BorderRadius.circular(999),
      trackGradient: null,
      trackColor: cs.onSurface.withOpacity(.08),
      trackBorder: Border.all(color: cs.onSurface.withOpacity(.16)),
      pillGradient: LinearGradient(
        begin: Alignment.topCenter, end: Alignment.bottomCenter,
        colors: [Colors.white.withOpacity(.50), Colors.white.withOpacity(.34)],
      ),
      pillColor: Colors.transparent,
      pillBorderColor: cs.primary.withOpacity(.85),
      pillBorderWidth: 1.4,
      pillShadows: [BoxShadow(color: Colors.black.withOpacity(.10), blurRadius: 8, offset: const Offset(0,2))],
      labelColor: cs.onSurface.withOpacity(.92),
      selectedLabelColor: cs.primary,
      selectedWeight: FontWeight.w800,
    );
  }

  // Filled selected look (Material-ish)
  static SnapStyle filled(BuildContext ctx) {
    final cs = Theme.of(ctx).colorScheme;
    return SnapStyle(
      trackHeight: 32,
      pillHeight: 24,
      radius: BorderRadius.circular(14),
      trackGradient: null,
      trackColor: cs.surfaceVariant,
      trackBorder: Border.all(color: cs.outlineVariant),
      pillGradient: null,
      pillColor: cs.primary.withOpacity(.14),
      pillBorderColor: cs.primary.withOpacity(.55),
      pillBorderWidth: 1,
      pillShadows: const [],
      labelColor: cs.onSurfaceVariant,
      selectedLabelColor: cs.primary,
      selectedWeight: FontWeight.w700,
    );
  }

  // Subtle glass (brighter than before, no blur; works great on web)
  static SnapStyle glass(BuildContext ctx) {
    final cs = Theme.of(ctx).colorScheme;
    return SnapStyle(
      trackHeight: 34,
      pillHeight: 24,
      radius: BorderRadius.circular(999),
      trackGradient: LinearGradient(
        begin: Alignment.topCenter, end: Alignment.bottomCenter,
        colors: [
          cs.onSurface.withOpacity(.22),
          cs.onSurface.withOpacity(.12),
          cs.onSurface.withOpacity(.22),
        ],
      ),
      trackColor: Colors.transparent,
      trackBorder: Border.all(color: cs.onSurface.withOpacity(.26)),
      pillGradient: LinearGradient(
        begin: Alignment.topCenter, end: Alignment.bottomCenter,
        colors: [Colors.white.withOpacity(.44), Colors.white.withOpacity(.30), Colors.white.withOpacity(.44)],
      ),
      pillColor: Colors.transparent,
      pillBorderColor: cs.primary.withOpacity(.85),
      pillBorderWidth: 1.6,
      pillShadows: [BoxShadow(color: Colors.black.withOpacity(.12), blurRadius: 10, offset: const Offset(0,3))],
      labelColor: cs.onSurface.withOpacity(.92),
      selectedLabelColor: cs.primary,
      selectedWeight: FontWeight.w800,
    );
  }

  // Ultra-minimal underline indicator (no pill) – modern “tab” vibe
  static SnapStyle underline(BuildContext ctx) {
    final cs = Theme.of(ctx).colorScheme;
    return SnapStyle(
      trackHeight: 36,
      pillHeight: 2, // not used; we render a 2px underline as "pill"
      radius: BorderRadius.circular(0),
      trackGradient: null,
      trackColor: Colors.transparent,
      trackBorder: Border(
        bottom: BorderSide(color: cs.outlineVariant, width: 1),
        top: BorderSide(color: Colors.transparent, width: 0),
      ),
      pillGradient: null,
      pillColor: Colors.transparent,
      pillBorderColor: Colors.transparent,
      pillBorderWidth: 0,
      pillShadows: const [],
      labelColor: cs.onSurface.withOpacity(.85),
      selectedLabelColor: cs.primary,
      selectedWeight: FontWeight.w800,
    );
  }
}

class SnapSlider extends StatefulWidget {
  final List<String> labels;
  final int initialIndex;
  final ValueChanged<int> onChanged;
  final SnapStyle? style;
  const SnapSlider({
    super.key,
    required this.labels,
    required this.initialIndex,
    required this.onChanged,
    this.style,
  });
  @override
  State<SnapSlider> createState() => _SnapSliderState();
}

class _SnapSliderState extends State<SnapSlider> {
  late int _index = widget.initialIndex.clamp(0, widget.labels.length - 1);
  bool _dragging = false;
  double _dragLeft = 0;

  @override
  Widget build(BuildContext context) {
    final s = widget.style ?? SnapStyles.minimal(context);

    return LayoutBuilder(builder: (context, c) {
      final width = c.maxWidth;
      final count = widget.labels.length;
      final segW = width / count;

      final trackH = s.trackHeight;
      final thumbH = s.pillHeight;
      final thumbW = (segW - 16).clamp(84.0, 220.0);
      final maxLeft = width - thumbW;

      double leftFor(int i) => (i * segW) + (segW - thumbW) / 2;
      int hoverIdx() {
        if (!_dragging) return _index;
        final center = (_dragLeft + thumbW / 2).clamp(0.0, width);
        return (center / segW).round().clamp(0, count - 1);
      }
      final hi = hoverIdx();

      // Underline style path
      if (identical(s, SnapStyles.underline(context))) {
        return SizedBox(
          height: trackH,
          child: Stack(children: [
            Container(
              decoration: BoxDecoration(color: s.trackColor, border: s.trackBorder),
            ),
            Row(
              children: List.generate(count, (i) {
                final sel = i == hi;
                return Expanded(
                  child: InkWell(
                    onTap: () => setState(() { _index = i; widget.onChanged(i); }),
                    child: Stack(children: [
                      Center(child: Text(
                        widget.labels[i],
                        style: TextStyle(
                          fontWeight: sel ? s.selectedWeight : FontWeight.w600,
                          color: sel ? s.selectedLabelColor : s.labelColor,
                        ),
                      )),
                      if (i == _index)
                        Align(
                          alignment: Alignment.bottomCenter,
                          child: Container(height: 2, margin: const EdgeInsets.symmetric(horizontal: 12),
                            color: s.selectedLabelColor),
                        ),
                    ]),
                  ),
                );
              }),
            ),
          ]),
        );
      }

      // Regular pill styles
      return SizedBox(
        height: trackH,
        child: Stack(children: [
          // Track
          Container(
            decoration: BoxDecoration(
              color: s.trackGradient == null ? s.trackColor : null,
              gradient: s.trackGradient,
              borderRadius: s.radius,
              border: s.trackBorder,
            ),
          ),

          // Labels (don’t move)
          Row(children: List.generate(count, (i) {
            final sel = i == hi;
            return Expanded(
              child: InkWell(
                borderRadius: s.radius,
                onTap: () => setState(() { _index = i; widget.onChanged(i); }),
                child: Center(
                  child: AnimatedDefaultTextStyle(
                    duration: const Duration(milliseconds: 120),
                    style: TextStyle(
                      fontWeight: sel ? s.selectedWeight : FontWeight.w600,
                      color: sel ? s.selectedLabelColor : s.labelColor,
                    ),
                    child: Text(widget.labels[i]),
                  ),
                ),
              ),
            );
          })),

          // Pill
          Positioned(
            left: (_dragging ? _dragLeft : leftFor(_index)).clamp(0.0, maxLeft),
            top: (trackH - thumbH) / 2,
            child: GestureDetector(
              behavior: HitTestBehavior.opaque,
              onHorizontalDragStart: (_) { setState(() { _dragging = true; _dragLeft = leftFor(_index); }); },
              onHorizontalDragUpdate: (d) { setState(() { _dragLeft = (_dragLeft + d.delta.dx).clamp(0.0, maxLeft); }); },
              onHorizontalDragEnd: (_) {
                final center = (_dragLeft + thumbW / 2).clamp(0.0, width);
                final idx = (center / segW).round().clamp(0, count - 1);
                setState(() { _dragging = false; _index = idx; });
                widget.onChanged(idx);
              },
              child: AnimatedContainer(
                duration: _dragging ? Duration.zero : const Duration(milliseconds: 140),
                curve: Curves.easeOut,
                width: thumbW,
                height: thumbH,
                decoration: BoxDecoration(
                  color: s.pillGradient == null ? s.pillColor : null,
                  gradient: s.pillGradient,
                  borderRadius: s.radius,
                  border: Border.all(color: s.pillBorderColor, width: s.pillBorderWidth),
                  boxShadow: s.pillShadows,
                ),
              ),
            ),
          ),
        ]),
      );
    });
  }
}


/* ============================================================================
   TIMED MODE — Digital SAT “exam skin” + feedback UX
============================================================================ */

class Attempt {
  final Question q;
  final String? chosenLabel;
  final int timeMs;
  final bool flagged;
  Attempt(this.q, this.chosenLabel, this.timeMs, {this.flagged = false});
  bool get correct => chosenLabel == q.correctLabel;

  Map<String, dynamic> toMap() {
  return {
    'questionId': q.id,
    'prompt': q.prompt,
    'category': q.category,
    'subcategory': q.subcategory,
    'section': q.section,
    'chosenLabel': chosenLabel,
    'correctLabel': q.correctLabel,
    'correct': correct,
    'timeMs': timeMs,
    'flagged': flagged,
  };
}
  
}

class TimedModePage extends StatefulWidget {
  final int totalSeconds; // 300/600/900
  final SectionMode sectionMode;
  const TimedModePage({super.key, required this.totalSeconds, required this.sectionMode});
  @override State<TimedModePage> createState() => _TimedModePageState();
}

class _TimedModePageState extends State<TimedModePage> {
  final _repo = LocalQuestionRepo();
  static const Color _satBlue = Color(0xFF0F5CC8);

  late int _secondsLeft = widget.totalSeconds;
  Timer? _overallTimer;

  List<Question> _deck = [];
  Question? _current;

  final List<Attempt> _attempts = [];
  final Stopwatch _qWatch = Stopwatch();
  bool _ended = false;
  bool _exhausted = false;

  // exam toolbar state
  bool _flagged = false;
  double _fontScale = 1.0;

  // feedback state
  String? _selectedLabel;   // which option was tapped
  bool _selectedCorrect = false;
  bool _showingFeedback = false;

  String _mmss(int secs) {
    final m = secs ~/ 60;
    final s = secs % 60;
    return '$m:${s.toString().padLeft(2, '0')}';
  }

  Set<String>? get _filter {
    switch (widget.sectionMode) {
      case SectionMode.math: return {'Math'};
      case SectionMode.reading: return {'Reading'};
      case SectionMode.writing: return {'Writing'};
      case SectionMode.mixed: return null;
    }
  }

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    final list = await _repo.fetch(sections: _filter);
    if (!mounted) return;

    _deck = List<Question>.from(list)..shuffle(Random());

    if (_deck.isEmpty) {
      _endGame(exhausted: true);
      return;
    }

    _startOverallTimer();
    _nextQuestion();
    setState(() {});
  }

  void _startOverallTimer() {
    _overallTimer = Timer.periodic(const Duration(seconds: 1), (_) {
      if (!mounted) return;
      setState(() => _secondsLeft--);
      if (_secondsLeft <= 0) _endGame();
    });
  }

  void _nextQuestion() {
    _qWatch..stop()..reset();

    if (_deck.isEmpty) {
      _endGame(exhausted: true);
      return;
    }

    _current = _deck.removeAt(0);
    _flagged = false;

    // reset feedback visuals
    _selectedLabel = null;
    _selectedCorrect = false;
    _showingFeedback = false;

    _qWatch.start();
    setState(() {});
  }

  Future<void> _answer(AnswerOption opt) async {
    if (_ended || _current == null || _showingFeedback) return;

    _qWatch.stop();
    final correct = opt.isCorrect;

    // record attempt
    _attempts.add(Attempt(_current!, opt.label, _qWatch.elapsedMilliseconds, flagged: _flagged));

    // set feedback state (colors/animations lock in)
    setState(() {
      _selectedLabel = opt.label;
      _selectedCorrect = correct;
      _showingFeedback = true;
    });

    // haptics (mobile); web ignores gracefully
    try {
      if (!kIsWeb) {
        if (correct) {
          HapticFeedback.lightImpact();
        } else {
          HapticFeedback.mediumImpact(); // a bit stronger for wrong
        }
      }
    } catch (_) {}

    // hold ~1s so student can register result
    await Future.delayed(const Duration(milliseconds: 1000));
    if (!mounted || _ended) return;

    _nextQuestion();
  }

  Future<void> _saveAttemptsToFirebase() async {
    final sessionId = DateTime.now().millisecondsSinceEpoch.toString();

    final maps = _attempts.map((a) => a.toMap()).toList();

    await AttemptService().saveAttempts(
      maps,
      sessionId: sessionId,
      totalSeconds: widget.totalSeconds,
      sectionMode: widget.sectionMode.name,
    );
  }


  void _endGame({bool exhausted = false}) async {
    if (_ended) return;
    _ended = true;
    _exhausted = exhausted;
    _overallTimer?.cancel();
    _qWatch.stop();

  // ⬇️ SAVE ATTEMPTS TO FIREBASE BEFORE SHOWING DIALOG
  await _saveAttemptsToFirebase();


    final int attempted = _attempts.length;
    final int correct = _attempts.where((a) => a.correct).length;
    final double pct = attempted == 0 ? 0.0 : (100.0 * correct / attempted);
    final int avgMs = attempted == 0
        ? 0
        : _attempts.map((a) => a.timeMs).reduce((a, b) => a + b) ~/ attempted;

    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) => AlertDialog(
        title: Text(_exhausted ? 'All Questions Answered!' : 'Time’s up!'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (_exhausted) ...[
              const Text('You reached the end of the question bank for this mode. Nice work!'),
              const SizedBox(height: 12),
            ],
            Text('Correct: $correct / $attempted (${pct.toStringAsFixed(0)}%)'),
            const SizedBox(height: 8),
            Text('Questions answered: $attempted'),
            const SizedBox(height: 8),
            Text('Avg time per question: ${(avgMs / 1000).toStringAsFixed(1)}s'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.pop(context); // close dialog
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => ReviewPage(
                    attempts: List<Attempt>.from(_attempts),
                    totalSeconds: widget.totalSeconds,
                    sectionMode: widget.sectionMode,
                  ),
                ),
              );
            },
            child: const Text('Review Questions'),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(context);
              Navigator.pushReplacement(
                context,
                MaterialPageRoute(
                  builder: (_) => TimedModePage(
                    totalSeconds: widget.totalSeconds,
                    sectionMode: widget.sectionMode,
                  ),
                ),
              );
            },
            child: const Text('Play Again'),
          ),
          FilledButton(
            onPressed: () {
              Navigator.pop(context); // close dialog
              Navigator.pop(context); // exit to home
            },
            child: const Text('Exit'),
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    _overallTimer?.cancel();
    _qWatch.stop();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // SAT exam look only here
    final satTheme = Theme.of(context).copyWith(
      colorScheme: ColorScheme.fromSeed(seedColor: _satBlue, brightness: Brightness.light),
      progressIndicatorTheme: const ProgressIndicatorThemeData(color: _satBlue),
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(backgroundColor: _satBlue, foregroundColor: Colors.white),
      ),
    );

    final ready = _current != null;
    final total = widget.totalSeconds;
    final progress = _secondsLeft.clamp(0, total) / total;

    return Theme(
      data: satTheme,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Timed Mode'),
          actions: [IconButton(tooltip: 'End now', onPressed: _endGame, icon: const Icon(Icons.stop_circle_outlined))],
        ),
        body: !ready
            ? const Center(child: CircularProgressIndicator())
            : Padding(
                padding: const EdgeInsets.fromLTRB(16, 10, 16, 16),
                child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
                  _ExamToolbar(
                    sectionMode: widget.sectionMode,
                    timerLabel: _mmss(_secondsLeft),
                    flagged: _flagged,
                    onToggleFlag: () => setState(() => _flagged = !_flagged),
                    onZoomOut: () => setState(() => _fontScale = (_fontScale - 0.1).clamp(0.8, 1.6)),
                    onZoomIn: () => setState(() => _fontScale = (_fontScale + 0.1).clamp(0.8, 1.6)),
                    onCalc: widget.sectionMode == SectionMode.math
                        ? () => showDialog(
                              context: context,
                              builder: (_) => const AlertDialog(
                                title: Text('Calculator'),
                                content: Text('Calculator UI coming soon.'),
                              ),
                            )
                        : null,
                  ),
                  const SizedBox(height: 10),

                  ClipRRect(
                    borderRadius: BorderRadius.circular(8),
                    child: LinearProgressIndicator(value: progress, minHeight: 8),
                  ),
                  const SizedBox(height: 16),

                  Expanded(
                    child: MediaQuery(
                      data: MediaQuery.of(context).copyWith(textScaleFactor: _fontScale),
                      child: Center(
                        child: ConstrainedBox(
                          constraints: const BoxConstraints(maxWidth: 980),
                          child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
                            _QuestionCard(current: _current!),
                            const SizedBox(height: 8),
                            ..._current!.options.map(
                              (opt) => _OptionRow(
                                letter: opt.label,
                                text: opt.text,
                                onTap: () => _answer(opt),
                                selected: _selectedLabel == opt.label,
                                showFeedback: _showingFeedback,
                                correctSelected: _selectedCorrect,
                                shake: _showingFeedback && _selectedLabel == opt.label && !_selectedCorrect,
                              ),
                            ),
                            const Spacer(),
                          ]),
                        ),
                      ),
                    ),
                  ),
                ]),
              ),
      ),
    );
  }
}

/* ============================================================================
   REVIEW PAGE  — SAT-style polish
============================================================================ */

class ReviewPage extends StatelessWidget {
  final List<Attempt> attempts;
  final int totalSeconds;
  final SectionMode sectionMode;

  const ReviewPage({
    super.key,
    required this.attempts,
    required this.totalSeconds,
    required this.sectionMode,
  });

  void _goHome(BuildContext context) {
    Navigator.popUntil(context, (route) => route.isFirst);
  }

  void _playAgain(BuildContext context) {
    Navigator.pushAndRemoveUntil(
      context,
      MaterialPageRoute(
        builder: (_) => TimedModePage(
          totalSeconds: totalSeconds,
          sectionMode: sectionMode,
        ),
      ),
      (route) => route.isFirst,
    );
  }

  @override
  Widget build(BuildContext context) {
    const satBlue = Color(0xFF0F5CC8);
    final satTheme = Theme.of(context).copyWith(
      colorScheme: ColorScheme.fromSeed(seedColor: satBlue, brightness: Brightness.light),
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(backgroundColor: satBlue, foregroundColor: Colors.white),
      ),
      progressIndicatorTheme: const ProgressIndicatorThemeData(color: satBlue),
    );

    final int attempted = attempts.length;
    final int correct = attempts.where((a) => a.correct).length;
    final double pct = attempted == 0 ? 0.0 : (100.0 * correct / attempted);
    final int avgMs = attempted == 0
        ? 0
        : attempts.map((a) => a.timeMs).reduce((a, b) => a + b) ~/ attempted;
    final int flaggedCount = attempts.where((a) => a.flagged).length;

    return Theme(
      data: satTheme,
      child: WillPopScope(
        onWillPop: () async {
          _goHome(context);
          return false;
        },
        child: Scaffold(
          appBar: AppBar(
            title: const Text('Review Answers'),
            leading: IconButton(
              tooltip: 'Home',
              icon: const Icon(Icons.home),
              onPressed: () => _goHome(context),
            ),
            actions: [
              IconButton(
                tooltip: 'Play Again',
                icon: const Icon(Icons.refresh),
                onPressed: () => _playAgain(context),
              ),
            ],
          ),
          body: Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 1000),
              child: ListView.separated(
                padding: const EdgeInsets.fromLTRB(16, 16, 16, 24),
                itemCount: attempts.length + 1,
                separatorBuilder: (_, __) => const SizedBox(height: 12),
                itemBuilder: (context, i) {
                  if (i == 0) {
                    return _SummaryCard(
                      attempted: attempted,
                      correct: correct,
                      accuracyPct: pct,
                      avgSeconds: avgMs / 1000.0,
                      flagged: flaggedCount,
                    );
                  }
                  final a = attempts[i - 1];
                  return _ResultCard(a: a);
                },
              ),
            ),
          ),
          bottomNavigationBar: SafeArea(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: FilledButton.icon(
                onPressed: () => _playAgain(context),
                icon: const Icon(Icons.play_arrow),
                label: const Text('Play Again'),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

/* ============================================================================
   SMALL UI HELPERS (exam toolbar, question card, option rows, review widgets)
============================================================================ */

class _ExamToolbar extends StatelessWidget {
  final SectionMode sectionMode;
  final String timerLabel;
  final bool flagged;
  final VoidCallback onToggleFlag;
  final VoidCallback onZoomOut;
  final VoidCallback onZoomIn;
  final VoidCallback? onCalc;

  const _ExamToolbar({
    required this.sectionMode,
    required this.timerLabel,
    required this.flagged,
    required this.onToggleFlag,
    required this.onZoomOut,
    required this.onZoomIn,
    this.onCalc,
  });

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final sectionText = switch (sectionMode) {
      SectionMode.math => 'Math',
      SectionMode.reading => 'Reading',
      SectionMode.writing => 'Writing',
      SectionMode.mixed => 'Mixed',
    };

    return Row(
      children: [
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
          decoration: BoxDecoration(
            color: cs.secondaryContainer,
            borderRadius: BorderRadius.circular(999),
          ),
          child: Text(sectionText, style: TextStyle(color: cs.onSecondaryContainer, fontWeight: FontWeight.w700)),
        ),
        const Spacer(),
        if (onCalc != null)
          IconButton(
            tooltip: 'Calculator',
            onPressed: onCalc,
            icon: const Icon(Icons.calculate_outlined),
          ),
        IconButton(
          tooltip: flagged ? 'Unflag' : 'Flag for review',
          onPressed: onToggleFlag,
          icon: Icon(flagged ? Icons.flag : Icons.outlined_flag),
          color: flagged ? Colors.orange : null,
        ),
        IconButton(tooltip: 'A−', onPressed: onZoomOut, icon: const Icon(Icons.text_decrease)),
        IconButton(tooltip: 'A+', onPressed: onZoomIn, icon: const Icon(Icons.text_increase)),
        const SizedBox(width: 8),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
          decoration: BoxDecoration(
            color: cs.primaryContainer,
            borderRadius: BorderRadius.circular(999),
            border: Border.all(color: Theme.of(context).colorScheme.primary.withOpacity(.35)),
          ),
          child: Row(children: [
            Icon(Icons.timer, size: 18, color: cs.onPrimaryContainer),
            const SizedBox(width: 6),
            Text(timerLabel, style: TextStyle(color: cs.onPrimaryContainer, fontWeight: FontWeight.w800)),
          ]),
        ),
      ],
    );
  }
}

class _QuestionCard extends StatelessWidget {
  final Question current;
  const _QuestionCard({required this.current});

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return Card(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 16, 16, 12),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('[${current.section}] ${current.category} · ${current.subcategory}',
              style: Theme.of(context).textTheme.labelLarge?.copyWith(color: cs.onSurfaceVariant)),
          const SizedBox(height: 8),
          Text(current.prompt,
              style: Theme.of(context).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w700)),
        ]),
      ),
    );
  }
}

class _OptionRow extends StatefulWidget {
  final String letter;
  final String text;
  final VoidCallback onTap;

  // feedback props
  final bool selected;
  final bool showFeedback;
  final bool correctSelected;
  final bool shake;

  const _OptionRow({
    required this.letter,
    required this.text,
    required this.onTap,
    required this.selected,
    required this.showFeedback,
    required this.correctSelected,
    required this.shake,
  });

  @override
  State<_OptionRow> createState() => _OptionRowState();
}

class _OptionRowState extends State<_OptionRow> with SingleTickerProviderStateMixin {
  late final AnimationController _ac =
      AnimationController(vsync: this, duration: const Duration(milliseconds: 380));
  late final Animation<double> _shake = TweenSequence<double>([
    TweenSequenceItem(tween: Tween(begin: 0, end: -6), weight: 1),
    TweenSequenceItem(tween: Tween(begin: -6, end: 6), weight: 2),
    TweenSequenceItem(tween: Tween(begin: 6, end: -4), weight: 2),
    TweenSequenceItem(tween: Tween(begin: -4, end: 0), weight: 1),
  ]).animate(CurvedAnimation(parent: _ac, curve: Curves.easeOut));

  @override
  void didUpdateWidget(covariant _OptionRow old) {
    super.didUpdateWidget(old);
    if (widget.shake && !old.shake) {
      _ac
        ..reset()
        ..forward();
    }
  }

  @override
  void dispose() {
    _ac.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;

    // default
    Color fill = Colors.transparent;
    Color border = cs.outlineVariant;
    Color dotBg = cs.secondaryContainer;
    Color dotFg = cs.onSecondaryContainer;

    // selected feedback tint
    if (widget.showFeedback && widget.selected) {
      final base = widget.correctSelected ? Colors.green : Colors.red;
      fill = base.withOpacity(.18);
      border = base.withOpacity(.60);
      dotBg = base.withOpacity(.20);
      dotFg = base.withOpacity(.90);
    }

    final row = Card(
      child: InkWell(
        borderRadius: BorderRadius.circular(16),
        onTap: widget.showFeedback ? null : widget.onTap,
        child: Container(
          decoration: BoxDecoration(
            color: fill,
            borderRadius: BorderRadius.circular(16),
            border: Border.all(color: border, width: widget.showFeedback && widget.selected ? 2 : 1),
          ),
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
          child: ListTile(
            leading: CircleAvatar(
              radius: 16,
              backgroundColor: dotBg,
              foregroundColor: dotFg,
              child: Text(widget.letter, style: const TextStyle(fontWeight: FontWeight.w800)),
            ),
            title: Text(widget.text),
          ),
        ),
      ),
    );

    return AnimatedBuilder(
      animation: _shake,
      builder: (_, child) => Transform.translate(
        offset: Offset(widget.shake ? _shake.value : 0, 0),
        child: child,
      ),
      child: row,
    );
  }
}

/* ----------------------- Review page widgets ----------------------------- */

class _SummaryCard extends StatelessWidget {
  final int attempted;
  final int correct;
  final double accuracyPct;
  final double avgSeconds;
  final int flagged;

  const _SummaryCard({
    required this.attempted,
    required this.correct,
    required this.accuracyPct,
    required this.avgSeconds,
    required this.flagged,
  });

  @override
  Widget build(BuildContext context) {
    final t = Theme.of(context);
    final cs = t.colorScheme;

    Widget chip(IconData icon, String label, {Color? bg, Color? fg}) {
      return Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        decoration: BoxDecoration(
          color: bg ?? cs.primaryContainer,
          borderRadius: BorderRadius.circular(999),
          border: Border.all(color: (fg ?? cs.onPrimaryContainer).withOpacity(.25)),
        ),
        child: Row(mainAxisSize: MainAxisSize.min, children: [
          Icon(icon, size: 18, color: fg ?? cs.onPrimaryContainer),
          const SizedBox(width: 8),
          Text(label, style: t.textTheme.labelLarge?.copyWith(color: fg ?? cs.onPrimaryContainer, fontWeight: FontWeight.w800)),
        ]),
      );
    }

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text('Summary', style: t.textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w800)),
          const SizedBox(height: 12),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children: [
              chip(Icons.verified_rounded, 'Score: $correct / $attempted'),
              chip(Icons.percent_rounded, 'Accuracy: ${accuracyPct.toStringAsFixed(0)}%'),
              chip(Icons.timer, 'Avg: ${avgSeconds.toStringAsFixed(1)}s'),
              chip(Icons.flag_rounded, 'Flagged: $flagged',
                  bg: cs.secondaryContainer, fg: cs.onSecondaryContainer),
            ],
          ),
        ]),
      ),
    );
  }
}

class _ResultCard extends StatelessWidget {
  final Attempt a;
  const _ResultCard({required this.a});

  @override
  Widget build(BuildContext context) {
    final t = Theme.of(context);
    final cs = t.colorScheme;

    final isCorrect = a.correct;

    Color tagBg;
    Color tagFg;
    IconData tagIcon;

    if (isCorrect) {
      tagBg = Colors.green.withOpacity(.18);
      tagFg = Colors.green.shade800;
      tagIcon = Icons.check_circle;
    } else {
      tagBg = Colors.red.withOpacity(.18);
      tagFg = Colors.red.shade800;
      tagIcon = Icons.cancel;
    }

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(
            children: [
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
                decoration: BoxDecoration(
                  color: tagBg,
                  borderRadius: BorderRadius.circular(999),
                  border: Border.all(color: tagFg.withOpacity(.35)),
                ),
                child: Row(children: [
                  Icon(tagIcon, size: 18, color: tagFg),
                  const SizedBox(width: 6),
                  Text(isCorrect ? 'Correct' : 'Incorrect',
                      style: t.textTheme.labelLarge?.copyWith(color: tagFg, fontWeight: FontWeight.w800)),
                ]),
              ),
              const Spacer(),
              if (a.flagged)
                Row(children: [
                  const Icon(Icons.flag, color: Colors.orange, size: 18),
                  const SizedBox(width: 6),
                  Text('Flagged', style: t.textTheme.labelLarge),
                ]),
            ],
          ),
          const SizedBox(height: 10),

          Text('[${a.q.section}] ${a.q.category} · ${a.q.subcategory}',
              style: t.textTheme.labelLarge?.copyWith(color: cs.onSurfaceVariant)),
          const SizedBox(height: 8),

          Text(a.q.prompt, style: t.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700)),
          const SizedBox(height: 12),

          for (final opt in a.q.options)
            _AnswerRow(
              label: opt.label,
              text: opt.text,
              isCorrect: opt.label == a.q.correctLabel,
              isChosen: opt.label == a.chosenLabel,
            ),

          const SizedBox(height: 8),
          Row(
            children: [
              Icon(Icons.schedule, size: 18, color: cs.onSurfaceVariant),
              const SizedBox(width: 6),
              Text('Time: ${(a.timeMs / 1000).toStringAsFixed(1)}s',
                  style: t.textTheme.bodyMedium?.copyWith(color: cs.onSurfaceVariant)),
            ],
          ),
        ]),
      ),
    );
  }
}

class _AnswerRow extends StatelessWidget {
  final String label;
  final String text;
  final bool isCorrect;
  final bool isChosen;

  const _AnswerRow({
    required this.label,
    required this.text,
    required this.isCorrect,
    required this.isChosen,
  });

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;

    Color fill = Colors.transparent;
    Color border = cs.outlineVariant;
    IconData icon = Icons.radio_button_unchecked;
    Color iconColor = cs.onSurfaceVariant;

    if (isCorrect) {
      fill = Colors.green.withOpacity(.12);
      border = Colors.green.withOpacity(.45);
      icon = Icons.check_circle;
      iconColor = Colors.green.shade700;
    }

    if (isChosen && !isCorrect) {
      fill = Colors.red.withOpacity(.12);
      border = Colors.red.withOpacity(.45);
      icon = Icons.cancel;
      iconColor = Colors.red.shade700;
    }

    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      decoration: BoxDecoration(
        color: fill,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: border),
      ),
      child: ListTile(
        leading: CircleAvatar(
          radius: 16,
          backgroundColor: cs.secondaryContainer,
          foregroundColor: cs.onSecondaryContainer,
          child: Text(label, style: const TextStyle(fontWeight: FontWeight.w800)),
        ),
        title: Text(text),
        trailing: Icon(icon, color: iconColor, size: 22),
      ),
    );
  }
}
