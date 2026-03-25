import 'package:flutter/material.dart';

import '../models/mode_item.dart';
import '../models/announcement.dart';
import '../widgets/announcements_board.dart'; // Was announcements_strip.dart but that doesn't exist, was it just a placeholder?
import '../widgets/playing_card.dart';
import '../widgets/dots_indicator.dart';
import '../backdrops/backdrop_grid.dart';
import '../backdrops/backdrop_waves.dart';
import '../backdrops/backdrop_formulas.dart';
import '../backdrops/backdrop_honeycomb.dart';
import '../modes/timed_mode.dart';
import '../widgets/streak_header.dart';


enum BackdropStyle { grid, waves, formulas, honeycomb }
const kBackdrop = BackdropStyle.waves;

class HomeCarousel extends StatefulWidget {
  const HomeCarousel({super.key});

  @override
  State<HomeCarousel> createState() => _HomeCarouselState();
}

class _HomeCarouselState extends State<HomeCarousel>
    with SingleTickerProviderStateMixin {
  PageController? _pc;
  double _viewportFraction = .5;
  double _page = 0;
  int _hoverIndex = -1;

  late final AnimationController _bgCtrl =
      AnimationController(vsync: this, duration: const Duration(seconds: 5))
        ..repeat(reverse: true);

  final modes = const <ModeItem>[
    ModeItem(
      id: 'timed',
      icon: Icons.timer_outlined,
      title: 'Timed Mode',
      description: 'Beat the clock and rack up correct answers.',
      live: true,
    ),
    ModeItem(
      id: 'practice',
      icon: Icons.view_list_outlined,
      title: 'Practice Sets',
      description: 'Pick a topic or difficulty and grind smart.',
    ),
    ModeItem(
      id: 'adaptive',
      icon: Icons.auto_graph_outlined,
      title: 'Adaptive Drill',
      description: 'Difficulty adapts to your performance.',
    ),
    ModeItem(
      id: 'full',
      icon: Icons.article_outlined,
      title: 'Full-Length Test',
      description: 'Simulate a real Digital SAT experience.',
    ),
    ModeItem(
      id: 'review',
      icon: Icons.replay_outlined,
      title: 'Review Mistakes',
      description: 'Revisit questions you missed and master them.',
    ),
  ];

  final List<Announcement> _announcements = [
    Announcement(
      Icons.campaign_outlined,
      'Streaks launch next week',
      subtitle: 'Keep your daily practice alive to earn bonuses.',
    ),
    Announcement(
      Icons.new_releases_outlined,
      'New Writing set',
      subtitle: 'Parallel structure & misplaced modifiers',
    ),
    Announcement(
      Icons.insights_outlined,
      'Tip: Use estimation',
      subtitle: 'Eliminate obviously wrong answers fast.',
      tint: const Color(0xFF8E0C08),
    ),
  ];

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final w = MediaQuery.sizeOf(context).width;
    final newFrac = _fractionForWidth(w);
    if (_pc == null) {
      _viewportFraction = newFrac;
      _pc = PageController(viewportFraction: _viewportFraction)
        ..addListener(_onScroll);
    } else if ((newFrac - _viewportFraction).abs() > .001) {
      final current = _pc!.page ?? 0.0;
      _pc!..removeListener(_onScroll)..dispose();
      _viewportFraction = newFrac;
      _pc = PageController(
        viewportFraction: _viewportFraction,
        initialPage: current.round(),
      )..addListener(_onScroll);
      setState(() => _page = current);
    }
  }

  double _fractionForWidth(double w) {
    if (w >= 1280) return .34;
    if (w >= 1024) return .38;
    if (w >= 800) return .46;
    return .70;
  }

  void _onScroll() {
    final p = _pc!.page ?? 0.0;
    if (p != _page) setState(() => _page = p);
  }

  @override
  void dispose() {
    _pc?..removeListener(_onScroll)..dispose();
    _bgCtrl.dispose();
    super.dispose();
  }

  Future<void> _startTimed() async {
    final cfg = await showTimedSetup(context);
    if (cfg == null || !mounted) return;
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => TimedModePage(
          totalSeconds: cfg.totalSeconds,
          sectionMode: cfg.sectionMode,
        ),
      ),
    );
  }

  void _soon() {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Coming soon ✌️')),
    );
  }

  Widget _buildBackdrop() {
    switch (kBackdrop) {
      case BackdropStyle.grid:
        return BackdropGrid(animation: _bgCtrl);
      case BackdropStyle.waves:
        return BackdropWaves(animation: _bgCtrl);
      case BackdropStyle.formulas:
        return BackdropFormulas(animation: _bgCtrl);
      case BackdropStyle.honeycomb:
        return BackdropHoneycomb(animation: _bgCtrl);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Saphire SAT'),
        actions: const [
          Padding(
            padding: EdgeInsets.only(right: 12),
            child: Icon(Icons.person_outline),
          ),
        ],
      ),
      body: Stack(
        children: [
          Positioned.fill(child: _buildBackdrop()),
          Column(
            children: [
              // ——— Streak header (replaces announcements)
            const StreakHeader(streakDays: 4),
            const SizedBox(height: 10),

              Expanded(
                child: PageView.builder(
                  controller: _pc,
                  physics: const BouncingScrollPhysics(),
                  itemCount: modes.length,
                  itemBuilder: (_, i) {
                    final d = (_page - i).abs();
                    final isCenter = d < .5;
                    final scale =
                        (1 - d * .10).clamp(.90, 1.0);
                    return Center(
                      child: MouseRegion(
                        onEnter: (_) =>
                            setState(() => _hoverIndex = i),
                        onExit: (_) =>
                            setState(() => _hoverIndex = -1),
                        child: Transform.scale(
                          scale: scale,
                          child: PlayingCard(
                            item: modes[i],
                            highlighted:
                                isCenter || _hoverIndex == i,
                            onStart: modes[i].live
                                ? _startTimed
                                : _soon,
                          ),
                        ),
                      ),
                    );
                  },
                ),
              ),
              const SizedBox(height: 8),
              Dots(
                count: modes.length,
                index: _page.round().clamp(0, modes.length - 1),
                active: Colors.white,
                inactive: Colors.white.withOpacity(.35),
              ),
              const SizedBox(height: 16),
              const SafeArea(child: SizedBox(height: 2)),
            ],
          ),
        ],
      ),
    );
  }
}
