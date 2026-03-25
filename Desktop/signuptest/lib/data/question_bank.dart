import '../models/question.dart';
import 'questions_math.dart';
import 'questions_reading.dart';
import 'questions_writing.dart';

class QuestionBank {
  static List<Question> all() => [
        ...mathQuestions,
        ...readingQuestions,
        ...writingQuestions,
      ];

  static List<Question> bySections(Set<String> sections) =>
      all().where((q) => sections.contains(q.section)).toList();
}
