import 'dart:math' as math;
import 'dart:ui' as ui;
import 'package:flutter/material.dart';

class BackdropFormulas extends StatelessWidget {
  final Animation<double> animation;
  const BackdropFormulas({super.key, required this.animation});

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: animation,
      builder: (_, __) =>
          CustomPaint(painter: _FormulaPainter(t: animation.value)),
    );
  }
}

class _FormulaPainter extends CustomPainter {
  final double t;
  _FormulaPainter({required this.t});

  static const reds = [Color(0xFFB80F0A), Color(0xFFA10E09)];
  static const glyphs = [
    'π',
    'Σ',
    '√',
    '∫',
    'Δ',
    '≈',
    '≠',
    '≥',
    '≤',
    'f(x)',
    'log',
    'x²',
    'sin',
    'cos',
    'θ'
  ];

  @override
  void paint(Canvas canvas, Size size) {
    final base = Paint()
      ..shader = ui.Gradient.radial(
        Offset(size.width * .6, size.height * .35),
        size.shortestSide * .95,
        reds,
      );
    canvas.drawRect(Offset.zero & size, base);

    final veil = Paint()
      ..shader = ui.Gradient.linear(
        const Offset(0, 0),
        Offset(size.width, size.height),
        const [
          Color(0x05FFFFFF),
          Color(0x00000000),
          Color(0x08000000),
        ],
        const [0.0, .6, 1.0],
      );
    canvas.drawRect(Offset.zero & size, veil);

    for (int i = 0; i < 60; i++) {
      final g = glyphs[i % glyphs.length];
      final baseX = (i * 97) % size.width.toInt();
      final baseY = (i * 53) % size.height.toInt();
      final dx = 40 * math.sin(t * 2 * math.pi + i * .7);
      final dy = 28 * math.cos(t * 2 * math.pi + i * .5);
      final pos = Offset(
        baseX.toDouble() + dx - 20,
        baseY.toDouble() + dy - 20,
      );

      final opacity =
          0.05 + 0.08 * (0.5 + 0.5 * math.sin(i + t * 6.28));
      final tp = TextPainter(
        text: TextSpan(
          text: g,
          style: TextStyle(
            color: Colors.white.withOpacity(opacity),
            fontSize: 16 + (i % 6) * 3,
            fontWeight: FontWeight.w700,
            letterSpacing: .5,
          ),
        ),
        textDirection: TextDirection.ltr,
      )..layout();
      tp.paint(canvas, pos);
    }
  }

  @override
  bool shouldRepaint(_FormulaPainter oldDelegate) => oldDelegate.t != t;
}
