class AnswerOption {
  final String label; // 'A','B','C','D'
  final String text;
  final bool isCorrect;
  const AnswerOption({required this.label, required this.text, required this.isCorrect});
}

class Question {
  final String id;
  final String section;      // "Math" | "Reading" | "Writing"
  final String category;     // e.g., "Algebra", "Grammar"
  final String subcategory;  // e.g., "Linear Equations", "Comma Use"
  final int difficulty;      // 1–5
  final String prompt;
  final List<AnswerOption> options;
  final int? timeLimitSec;   // optional per-question cap (not used by timed mode)
  const Question({
    required this.id,
    required this.section,
    required this.category,
    required this.subcategory,
    required this.difficulty,
    required this.prompt,
    required this.options,
    this.timeLimitSec,
  });

  String get correctLabel => options.firstWhere((o) => o.isCorrect).label;
}
