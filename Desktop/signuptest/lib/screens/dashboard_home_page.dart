// The new "Home" dashboard: settings icon, search bar, announcements,
// recents, progress, streak, and stats.

import 'package:flutter/material.dart';
import '../widgets/streak_header.dart';
import '../widgets/announcements_board.dart';
import '../widgets/recents_row.dart';
import '../widgets/progress_card.dart';
import '../widgets/stats_grid.dart';
import '../models/announcement.dart';



class DashboardHomePage extends StatelessWidget {
  const DashboardHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    
    // ---------------- Delacarations ----------------
    final announcements = <Announcement>[
      Announcement(
        Icons.new_releases_outlined,
        'New Writing Set',
        subtitle: 'Parallel structure & misplaced modifiers',
      ),
      Announcement(
        Icons.bolt_outlined,
        'Speed Tip',
        subtitle: 'Use estimation to eliminate wrong answers quickly.',
      ),
      Announcement(
        Icons.campaign_outlined,
        'Weekly Challenge',
        subtitle: 'Finish 3 drills by Sunday for +150 XP',
      ),
    ];

    final recentModes = <String>['Timed Mode', 'Practice Sets', 'Adaptive Drill'];


    //---------------- Widget -----------------------------------------------------
    return LayoutBuilder(
      builder: (context, constraints) {
        return SingleChildScrollView(
          padding: const EdgeInsets.fromLTRB(16, 8, 1, 24),
      
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [

              // ---------------- HEADER ROW ----------------
              Row(
                children: [
                  // Search bar FIRST with flexible width
                  Expanded(
                    child: TextField(
                      textInputAction: TextInputAction.search,
                      decoration: InputDecoration(
                        hintText: 'Search practice, tips, or questions',
                        prefixIcon: const Icon(Icons.search),
                        filled: true,
                        fillColor: Colors.white.withOpacity(.92),
                        contentPadding:
                            const EdgeInsets.symmetric(vertical: 12),
                        border: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
                      onSubmitted: (query) {
                        // TODO search implementation
                      },
                    ),
                  ),

                  const SizedBox(width: 12),

                  // Right-side settings button
                  IconButton(
                    tooltip: 'Settings',
                    onPressed: () {},
                    icon: const Icon(Icons.settings, color: Colors.black54),
                    style: IconButton.styleFrom(
                      backgroundColor: Colors.white.withOpacity(.85),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                  ),
                ],
              ),

              const SizedBox(height: 16),

              // ---------------- STREAK ----------------
              const StreakHeader(streakDays: 4),

              const SizedBox(height: 16),


              // ---------------- RECENTS ----------------
              RecentsRow(
                items: recentModes,
                onTap: (label) {
                  // TODO: Route logic
                },
              ),

              const SizedBox(height: 16),

              // ---------------- PROGRESS ----------------
              const ProgressCard(
                title: 'Weekly Progress',
                subtitle: 'You’re on track for your goal',
                progress: 0.62,
                trailingLabel: '62%',
              ),

              const SizedBox(height: 16),


              // ---------------- ANNOUNCEMENTS ----------------
              AnnouncementsBoard(items: announcements),

              const SizedBox(height: 16),

              // ---------------- STATS GRID ----------------
              const StatsGrid(
                items: [
                  StatItem(
                      icon: Icons.check_circle_outline,
                      label: 'Accuracy',
                      value: '78%'),
                  StatItem(
                      icon: Icons.av_timer,
                      label: 'Avg Time',
                      value: '52s'),
                  StatItem(
                      icon: Icons.auto_graph,
                      label: 'Score Est.',
                      value: '1320'),
                  StatItem(
                      icon: Icons.stacked_line_chart,
                      label: 'Trend',
                      value: '+3.4%'),
                ],
              ),

            ],
          ),
        );
      },
    );
  }
}
