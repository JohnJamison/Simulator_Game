import 'package:flutter/material.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Dialog Sanity',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.indigo),
      ),
      home: const Home(),
    );
  }
}

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Sanity Test')),
      body: Center(
        child: ElevatedButton(
          child: const Text('Open Centered Dialog'),
          onPressed: () async {
            await showGeneralDialog(
              context: context,
              barrierDismissible: true,
              barrierLabel: 'Close',
              barrierColor: Colors.black54,
              transitionDuration: const Duration(milliseconds: 180),
              pageBuilder: (_, __, ___) {
                return Center(
                  child: Material(
                    color: Theme.of(context).colorScheme.surface,
                    elevation: 12,
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(24),
                    ),
                    child: const SizedBox(
                      width: 520,
                      child: Padding(
                        padding: EdgeInsets.all(16),
                        child: _SliderDemo(),
                      ),
                    ),
                  ),
                );
              },
              transitionBuilder: (_, a, __, child) {
                // ensure Animation<double>
                final fade = CurvedAnimation(parent: a, curve: Curves.easeOut);
                final scale =
                    Tween<double>(begin: 0.98, end: 1.0).animate(fade);
                return FadeTransition(
                  opacity: fade,
                  child: ScaleTransition(scale: scale, child: child),
                );
              },
            ); // <-- THIS was missing
          },
        ),
      ),
    );
  }
}

class _SliderDemo extends StatefulWidget {
  const _SliderDemo();
  @override
  State<_SliderDemo> createState() => _SliderDemoState();
}

class _SliderDemoState extends State<_SliderDemo> {
  final _labels = const ['Math', 'Reading', 'Writing', 'Mixed'];
  int _index = 3;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Row(children: [
          const SizedBox(width: 40),
          Expanded(
            child: Center(
              child: Text('Centered Dialog',
                  style: Theme.of(context).textTheme.titleLarge),
            ),
          ),
          IconButton(
            onPressed: () => Navigator.pop(context),
            icon: const Icon(Icons.close),
          )
        ]),
        const SizedBox(height: 12),
        _SnapSlider(
          labels: _labels,
          initialIndex: _index,
          onChanged: (i) => setState(() => _index = i),
        ),
        const SizedBox(height: 16),
        Text('Selected: ${_labels[_index]}'),
        const SizedBox(height: 12),
      ],
    );
  }
}

class _SnapSlider extends StatefulWidget {
  final List<String> labels;
  final int initialIndex;
  final ValueChanged<int> onChanged;
  const _SnapSlider({
    required this.labels,
    required this.initialIndex,
    required this.onChanged,
  });
  @override
  State<_SnapSlider> createState() => _SnapSliderState();
}

class _SnapSliderState extends State<_SnapSlider> {
  late int _index = widget.initialIndex.clamp(0, widget.labels.length - 1);
  bool _dragging = false;
  double _dragLeft = 0;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, c) {
      final width = c.maxWidth;
      final count = widget.labels.length;
      final segW = width / count;
      const trackH = 48.0;
      const thumbH = 38.0;
      final thumbW = (segW - 12).clamp(84.0, 200.0);
      final maxLeft = width - thumbW;
      double leftFor(int i) => (i * segW) + (segW - thumbW) / 2;

      return SizedBox(
        height: trackH,
        child: Stack(children: [
          // track
          Container(
            decoration: BoxDecoration(
              color: Theme.of(context)
                  .colorScheme
                  .surfaceVariant
                  .withOpacity(.72),
              borderRadius: BorderRadius.circular(24),
            ),
          ),
          // labels (tap jumps)
          Row(
            children: List.generate(count, (i) {
              final sel = i == _index;
              return Expanded(
                child: InkWell(
                  borderRadius: BorderRadius.circular(24),
                  onTap: () => setState(() {
                    _index = i;
                    widget.onChanged(i);
                  }),
                  child: Center(
                    child: AnimatedDefaultTextStyle(
                      duration: const Duration(milliseconds: 120),
                      style: TextStyle(
                        fontWeight: sel ? FontWeight.w700 : FontWeight.w600,
                      ),
                      child: Text(widget.labels[i]),
                    ),
                  ),
                ),
              );
            }),
          ),
          // thumb (drag + snap)
          Positioned(
            left: (_dragging ? _dragLeft : leftFor(_index)).clamp(0.0, maxLeft),
            top: (trackH - thumbH) / 2,
            child: GestureDetector(
              behavior: HitTestBehavior.opaque,
              onHorizontalDragStart: (_) {
                setState(() {
                  _dragging = true;
                  _dragLeft = leftFor(_index);
                });
              },
              onHorizontalDragUpdate: (d) {
                setState(() {
                  _dragLeft =
                      (_dragLeft + d.delta.dx).clamp(0.0, maxLeft);
                });
              },
              onHorizontalDragEnd: (_) {
                final center = (_dragLeft + thumbW / 2).clamp(0.0, width);
                final idx =
                    (center / (width / count)).round().clamp(0, count - 1);
                setState(() {
                  _dragging = false;
                  _index = idx;
                });
                widget.onChanged(idx);
              },
              child: AnimatedContainer(
                duration:
                    _dragging ? Duration.zero : const Duration(milliseconds: 140),
                curve: Curves.easeOut,
                width: thumbW,
                height: thumbH,
                alignment: Alignment.center,
                decoration: BoxDecoration(
                  color: Theme.of(context)
                      .colorScheme
                      .onSurface
                      .withOpacity(.07), // translucent
                  borderRadius: BorderRadius.circular(20),
                  border: Border.all(
                    color: Theme.of(context)
                        .colorScheme
                        .primary
                        .withOpacity(.65),
                    width: 1.6,
                  ),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(.08),
                      blurRadius: 8,
                      offset: const Offset(0, 2),
                    )
                  ],
                ),
                child: Text(
                  widget.labels[_index],
                  style: const TextStyle(fontWeight: FontWeight.w700),
                ),
              ),
            ),
          ),
        ]),
      );
    });
  }
}
