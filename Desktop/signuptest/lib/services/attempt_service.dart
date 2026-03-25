import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:firebase_auth/firebase_auth.dart';

class AttemptService {
  final _firestore = FirebaseFirestore.instance;

  Future<void> saveAttempts(List<Map<String, dynamic>> attemptMaps,
      {required String sessionId, required int totalSeconds, required String sectionMode}) async {

    final user = FirebaseAuth.instance.currentUser;
    if (user == null) return;

    final uid = user.uid;

    final batch = _firestore.batch();
    final attemptsRef = _firestore
        .collection('users')
        .doc(uid)
        .collection('attempts');

    for (final map in attemptMaps) {
      final doc = attemptsRef.doc();
      batch.set(doc, {
        ...map,
        'sessionId': sessionId,
        'timestamp': FieldValue.serverTimestamp(),
        'sectionMode': sectionMode,
        'totalSeconds': totalSeconds,
      });
    }

    await batch.commit();
  }
}
