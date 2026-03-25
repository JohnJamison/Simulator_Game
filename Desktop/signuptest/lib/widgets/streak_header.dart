import 'package:flutter/material.dart';

class StreakHeader extends StatelessWidget {
  final int streakDays;

  const StreakHeader({super.key, required this.streakDays});

  @override
  Widget build(BuildContext context) {
    final streakColor =
        streakDays >= 5 ? const Color(0xFFFFB300) : Colors.white;

    return Container(
      decoration: BoxDecoration(
        color: Colors.white.withOpacity(0.12),
        borderRadius: BorderRadius.circular(18),
        border: Border.all(color: Colors.white.withOpacity(0.2)),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            '🔥 $streakDays-Day Streak',
            textAlign: TextAlign.center,
            style: TextStyle(
              fontSize: 24.0,
              color: streakColor,
              fontWeight: FontWeight.w800,

            ),
          ),
        ],
      ),
    );
  }
}
