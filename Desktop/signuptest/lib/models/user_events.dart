// lib/models/user_event.dart
//
// One "row" of user behavior: e.g. answering a question, time spent, etc.

class UserEvent {
  /// When this event happened
  final DateTime timestamp;

  /// What kind of event:
  ///   "math_correct", "math_incorrect",
  ///   "reading_correct", "writing_time", etc.
  final String type;

  /// Numeric value for the event:
  ///   - 1 for "one question correct"
  ///   - 0 for incorrect (if you want)
  ///   - seconds spent (for *_time events)
  final double value;

  /// Optional topic / subcategory:
  ///   "algebra", "advanced_math", "data_analysis", etc.
  final String? subcategory;

  UserEvent({
    required this.timestamp,
    required this.type,
    required this.value,
    this.subcategory,
  });

  // For saving to JSON later
  Map<String, dynamic> toJson() => {
        'timestamp': timestamp.toIso8601String(),
        'type': type,
        'value': value,
        'subcategory': subcategory,
      };

  // For loading from JSON later
  factory UserEvent.fromJson(Map<String, dynamic> json) => UserEvent(
        timestamp: DateTime.parse(json['timestamp'] as String),
        type: json['type'] as String,
        value: (json['value'] as num).toDouble(),
        subcategory: json['subcategory'] as String?,
      );
}
