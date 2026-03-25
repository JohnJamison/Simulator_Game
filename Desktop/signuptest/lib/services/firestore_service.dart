import 'package:cloud_firestore/cloud_firestore.dart';

class FirestoreService {
  final FirebaseFirestore _firestore = FirebaseFirestore.instance;

  // Add a user with extended fields (friends, attempts, stats)
  Future<void> addUser(
    String id,
    String username,
    String displayName,
    String state,
    String city,
    String country,
    String email,
    String number,
    List<int> answers,
  ) async {
    await _firestore.collection('users').doc(id).set({
      'username': username,
      'displayName': displayName,
      'email': email,
      'number': number,

      // -- Location --
      'location': {
        'state': state,
        'city': city,
        'country': country,
      },

      // -- Answers (what you already had) --
      'answers': answers,

      // -- NEW: Friends list (userIds)
      'friends': <String>[], 

      // -- NEW: Attempt history --
      //    You can push objects here like:
      //    { "questionId": 14, "correct": true, "time": 12.4 }
      'attemptHistory': <Map<String, dynamic>>[],

      // -- NEW: Stats structure --
      'stats': {
        'totalAttempts': 0,
        'totalCorrect': 0,
        'avgTimeSec': 0.0,
        'mathCorrect': 0,
        'readingCorrect': 0,
        'writingCorrect': 0,
      },

      // Timestamp
      'createdAt': FieldValue.serverTimestamp(),
    });
  }

  // Update a user's fields
  Future<void> updateUser(
    String id, {
    String? username,
    String? displayName,
    String? state,
    String? city,
    String? country,
    String? email,
    String? number,
  }) async {
    Map<String, dynamic> updatedData = {};

    if (username != null) updatedData['username'] = username;
    if (displayName != null) updatedData['displayName'] = displayName;

    if (state != null || city != null || country != null) {
      updatedData['location'] = {};
      if (state != null) updatedData['location']['state'] = state;
      if (city != null) updatedData['location']['city'] = city;
      if (country != null) updatedData['location']['country'] = country;
    }

    if (email != null) updatedData['email'] = email;
    if (number != null) updatedData['number'] = number;

    if (updatedData.isNotEmpty) {
      await _firestore.collection('users').doc(id).update(updatedData);
    }
  }

  // Delete a user
  Future<void> deleteUser(String id) async {
    await _firestore.collection('users').doc(id).delete();
  }

  // Get user
  Future<Map<String, dynamic>?> getUser(String id) async {
    DocumentSnapshot snapshot =
        await _firestore.collection('users').doc(id).get();

    return snapshot.data() as Map<String, dynamic>?;
  }

  // Get user answers
  Future<List<int>?> getUserAnswers(String id) async {
    DocumentSnapshot snapshot =
        await _firestore.collection('users').doc(id).get();

    final data = snapshot.data() as Map<String, dynamic>?;
    if (data != null && data['answers'] != null) {
      return List<int>.from(data['answers']);
    }
    return null;
  }

  // Add answers to user
  Future<void> addAnswersToUser(String id, List<int> newAnswers) async {
    await _firestore.collection('users').doc(id).update({
      'answers': FieldValue.arrayUnion(newAnswers),
    });
  }

  // NEW: Add attempt
  Future<void> addAttempt(String id, Map<String, dynamic> attempt) async {
    await _firestore.collection('users').doc(id).update({
      'attemptHistory': FieldValue.arrayUnion([attempt]),
    });
  }

  // NEW: Update stats
  Future<void> updateStats(String id, Map<String, dynamic> newStats) async {
    await _firestore.collection('users').doc(id).update({
      'stats': newStats,
    });
  }

  // NEW: Add friend
  Future<void> addFriend(String id, String friendUid) async {
    await _firestore.collection('users').doc(id).update({
      'friends': FieldValue.arrayUnion([friendUid]),
    });
  }
}
