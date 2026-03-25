import 'dart:math' as math;
import 'dart:ui' as ui;
import 'package:flutter/material.dart';

class BackdropWaves extends StatelessWidget {
  final Animation<double> animation;
  const BackdropWaves({super.key, required this.animation});

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: animation,
      builder: (_, __) =>
          CustomPaint(painter: _WavesPainter(t: animation.value)),
    );
  }
}

class _WavesPainter extends CustomPainter {
  final double t;
  _WavesPainter({required this.t});

  Path _wave(Size size, double amp, double freq, double phase, double baseY) {
    final p = Path()..moveTo(0, baseY);
    for (double x = 0; x <= size.width; x += 8) {
      final y = baseY +
          amp *
              math.sin((x / size.width) * freq * 2 * math.pi + phase);
      p.lineTo(x, y);
    }
    p
      ..lineTo(size.width, size.height)
      ..lineTo(0, size.height)
      ..close();
    return p;
  }

  @override
  void paint(Canvas canvas, Size size) {
    final base = Paint()
      ..shader = ui.Gradient.linear(
        const Offset(0, 0),
        Offset(0, size.height),
        const [Color(0xFFB80F0A), Color(0xFF8E0C08)],
      );
    canvas.drawRect(Offset.zero & size, base);

    final layers = [
      (18.0, 1.5, 0.0 + t * 2.0, size.height * .35, const Color(0x22FFFFFF)),
      (24.0, 2.0, 1.4 + t * 1.6, size.height * .48, const Color(0x1FFFFFFF)),
      (30.0, 2.6, 2.2 + t * 1.2, size.height * .62, const Color(0x14FFFFFF)),
      (40.0, 3.2, 3.0 + t * 0.9, size.height * .78, const Color(0x10FFFFFF)),
    ];

    for (final (amp, freq, phase, baseY, color) in layers) {
      final paint = Paint()..color = color;
      canvas.drawPath(_wave(size, amp, freq, phase, baseY), paint);
    }
  }

  @override
  bool shouldRepaint(_WavesPainter oldDelegate) => oldDelegate.t != t;
}
