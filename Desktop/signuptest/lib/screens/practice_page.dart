import 'package:flutter/material.dart';
import '../modes/timed_mode.dart';  // <-- Correct path to your timed mode file

class PracticePage extends StatelessWidget {
  const PracticePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF8B0D07),
      appBar: AppBar(
        title: const Text("Practice Modes"),
        backgroundColor: const Color(0xFF8B0D07),
        elevation: 0,
      ),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              "Choose a Mode",
              style: TextStyle(
                fontSize: 26,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
            ),

            const SizedBox(height: 30),

            // ---------- TIMED MODE BUTTON ----------
            _modeButton(
              context: context,
              title: "Timed Mode",
              icon: Icons.timer_outlined,
              onTap: () async {
                // open the setup dialog from TimedMode
                final cfg = await showTimedSetup(context);
                if (cfg == null) return; // user canceled

                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (_) => TimedModePage(
                      totalSeconds: cfg.totalSeconds,
                      sectionMode: cfg.sectionMode,
                    ),
                  ),
                );
              },
            ),

            const SizedBox(height: 20),

            // ---------- OTHER MODES ----------
            _modeButton(
              context: context,
              title: "Classic Practice",
              icon: Icons.school_outlined,
              onTap: () {},
            ),

            const SizedBox(height: 20),

            _modeButton(
              context: context,
              title: "Lightning Mode",
              icon: Icons.flash_on_outlined,
              onTap: () {},
            ),

            const SizedBox(height: 20),

            _modeButton(
              context: context,
              title: "Streak Mode",
              icon: Icons.local_fire_department_outlined,
              onTap: () {},
            ),
          ],
        ),
      ),
    );
  }

  Widget _modeButton({
    required BuildContext context,
    required String title,
    required IconData icon,
    required VoidCallback onTap,
  }) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(16),
      child: Ink(
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.1),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: Colors.white24, width: 2),
        ),
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 20, horizontal: 18),
          child: Row(
            children: [
              Icon(icon, color: Colors.white, size: 28),
              const SizedBox(width: 20),
              Text(
                title,
                style: const TextStyle(
                  fontSize: 20,
                  color: Colors.white,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const Spacer(),
              const Icon(Icons.arrow_forward_ios, color: Colors.white, size: 20),
            ],
          ),
        ),
      ),
    );
  }
}
