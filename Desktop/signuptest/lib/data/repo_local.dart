import 'dart:async';
import '../models/question.dart';
import 'question_bank.dart';

/// Simple repository that can later be swapped for a real backend.
class LocalQuestionRepo {
  Future<List<Question>> fetch({Set<String>? sections}) async {
    // Slight delay to mimic I/O and keep API async-friendly.
    await Future<void>.delayed(const Duration(milliseconds: 40));
    final list = (sections == null || sections.isEmpty)
        ? QuestionBank.all()
        : QuestionBank.bySections(sections);
    // Return a copy so callers can shuffle without mutating the source.
    return List<Question>.from(list);
  }
}
