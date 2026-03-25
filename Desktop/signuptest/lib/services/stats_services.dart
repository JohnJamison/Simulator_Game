// lib/services/stats_service.dart
//
// Helpers to compute stats from a user's event history.

import '../models/user_profile.dart';
import '../models/user_events.dart';

enum QuestionSection { math, reading, writing }

class StatsService {
  // Helper: convert section enum to a base string like "math", "reading", etc.
  static String _sectionName(QuestionSection section) {
    switch (section) {
      case QuestionSection.math:
        return 'math';
      case QuestionSection.reading:
        return 'reading';
      case QuestionSection.writing:
        return 'writing';
    }
  }

  /// Log a question result as two events:
  ///   - one correct/incorrect event
  ///   - one time event (seconds spent)
  static void logQuestion({
    required UserProfile user,
    required QuestionSection section,
    required bool isCorrect,
    required double seconds,
    String? topic, // e.g. "algebra", "advanced_math"
  }) {
    final now = DateTime.now();
    final base = _sectionName(section);

    // Correct / incorrect event
    user.events.add(UserEvent(
      timestamp: now,
      type: isCorrect ? '${base}_correct' : '${base}_incorrect',
      value: 1,
      subcategory: topic,
    ));

    // Time event
    user.events.add(UserEvent(
      timestamp: now,
      type: '${base}_time',
      value: seconds,
      subcategory: topic,
    ));
  }

  /// Accuracy for a section over the last [window] duration.
  /// Example: accuracyFor(user, QuestionSection.math, window: Duration(days: 7))
  static double accuracyFor(
    UserProfile user,
    QuestionSection section, {
    Duration? window,
    String? topic,
  }) {
    final base = _sectionName(section);
    final cutoff =
        window == null ? null : DateTime.now().subtract(window);

    bool _inWindow(UserEvent e) =>
        cutoff == null || e.timestamp.isAfter(cutoff);

    bool _topicMatches(UserEvent e) =>
        topic == null || e.subcategory == topic;

    final correct = user.events.where((e) {
      return e.type == '${base}_correct' &&
          _inWindow(e) &&
          _topicMatches(e);
    }).length;

    final total = user.events.where((e) {
      final isCorrect = e.type == '${base}_correct';
      final isIncorrect = e.type == '${base}_incorrect';
      return (isCorrect || isIncorrect) &&
          _inWindow(e) &&
          _topicMatches(e);
    }).length;

    if (total == 0) return 0.0;
    return correct / total;
  }

  /// Average time per question for a section over [window].
  /// Example: avgTimeFor(user, QuestionSection.math, window: Duration(days: 30))
  static double avgTimeFor(
    UserProfile user,
    QuestionSection section, {
    Duration? window,
    String? topic,
  }) {
    final base = _sectionName(section);
    final cutoff =
        window == null ? null : DateTime.now().subtract(window);

    bool _inWindow(UserEvent e) =>
        cutoff == null || e.timestamp.isAfter(cutoff);

    bool _topicMatches(UserEvent e) =>
        topic == null || e.subcategory == topic;

    final times = user.events.where((e) {
      return e.type == '${base}_time' &&
          _inWindow(e) &&
          _topicMatches(e);
    }).map((e) => e.value).toList();

    if (times.isEmpty) return 0.0;

    final sum = times.reduce((a, b) => a + b);
    return sum / times.length;
  }

  /// Count total questions answered for a section over [window].
  static int totalQuestionsFor(
    UserProfile user,
    QuestionSection section, {
    Duration? window,
    String? topic,
  }) {
    final base = _sectionName(section);
    final cutoff =
        window == null ? null : DateTime.now().subtract(window);

    bool _inWindow(UserEvent e) =>
        cutoff == null || e.timestamp.isAfter(cutoff);

    bool _topicMatches(UserEvent e) =>
        topic == null || e.subcategory == topic;

    return user.events.where((e) {
      final isCorrect = e.type == '${base}_correct';
      final isIncorrect = e.type == '${base}_incorrect';
      return (isCorrect || isIncorrect) &&
          _inWindow(e) &&
          _topicMatches(e);
    }).length;
  }
}
