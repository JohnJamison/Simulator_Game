import 'package:flutter/material.dart';
import 'edit_profile_page.dart';
import '../../models/user_model.dart';

class ProfilePage extends StatelessWidget {
  const ProfilePage({super.key});

  @override
  Widget build(BuildContext context) {
    final user = UserModel.instance;

    return Scaffold(
      backgroundColor: Colors.white,
      body: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [

            // ------------------------- HEADER -------------------------
            // ------------------------- HEADER (photo left, text right) -------------------------
            Container(
              padding: const EdgeInsets.fromLTRB(20, 50, 20, 30),
              decoration: BoxDecoration(
                gradient: LinearGradient(
                  colors: [
                    const Color.fromARGB(255, 196, 18, 18),
                    const Color.fromARGB(255, 196, 18, 18).withOpacity(0.7),
                  ],
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                ),
              ),

              //--------------------  Header Row  --------------------
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [

                  // --------- PROFILE PHOTO ---------
                  CircleAvatar(
                    radius: 45,
                    backgroundColor: Colors.white,
                    child: Text(
                      user.displayInitials,
                      style: const TextStyle(
                        fontSize: 28,
                        fontWeight: FontWeight.bold,
                        color: Colors.black87,
                      ),
                    ),
                  ),

                  const SizedBox(width: 18),

                  // --------- NAME + USERNAME  ---------
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      // Display name + stats in a row
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          // Name + username block
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                user.name,
                                style: const TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(height: 3),
                              Text(
                                user.username,
                                style: TextStyle(
                                  fontSize: 14,
                                  color: Colors.white.withOpacity(0.8),
                                ),
                              ),
                            ],
                          ),
                        ],
                      ),
                    ],
                  ),

                  // Stats moved up next to the name
                  Expanded(
                    child: Center(
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          _statColumn("Accuracy", "${user.accuracy}%"),
                          const SizedBox(width: 40),
                          _statColumn("Avg Time", "${user.avgTime}s"),
                          const SizedBox(width: 40),
                          _statColumn("Estimate", "${user.scoreEstimate}"),
                        ],
                      ),
                    ),
                  ),

                  // -------- EDIT BUTTON RIGHT-ALIGNED --------
                  ElevatedButton(
                    onPressed: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(builder: (_) => const EditProfilePage()),
                      );
                    },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.white,
                      foregroundColor: Colors.black87,
                      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
                      shape: const StadiumBorder(),
                    ),
                    child: const Text("Edit Profile"),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 30),

            // =========================================================
            //                         FRIENDS
            // =========================================================
            _sectionTitle("Friends"),

            const SizedBox(height: 10),

            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Column(
                children: [
                  // Search bar
                  TextField(
                    decoration: InputDecoration(
                      prefixIcon: const Icon(Icons.search),
                      hintText: "Search friends...",
                      filled: true,
                      fillColor: const Color.fromARGB(255, 219, 213, 213),
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(14),
                        borderSide: BorderSide.none,
                      ),
                    ),
                  ),

                  const SizedBox(height: 16),

                  // Recently Active preview
                  SizedBox(
                    height: 90,
                    child: ListView(
                      scrollDirection: Axis.horizontal,
                      children: [
                        _friendBubble("Ava", "A"),
                        _friendBubble("Leo", "L"),
                        _friendBubble("Maya", "M"),
                        _friendBubble("Noah", "N"),
                      ],
                    ),
                  ),

                  const SizedBox(height: 10),

                  // View All Friends button
                  Align(
                    alignment: Alignment.centerRight,
                    child: TextButton(
                      onPressed: () => _showFriendsPopup(context),
                      child: const Text("View All"),
                    ),
                  ),
                ],
              ),
            ),

            const SizedBox(height: 30),

            // ---------------------------------------------------------
            //                    DETAILED STATS CARDS
            // ---------------------------------------------------------
            _sectionTitle("Your Stats"),

            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 20),
              child: Column(
                children: [
                  _statCard("Total Questions", user.totalPracticed.toString(),
                      Icons.assignment_outlined),
                  const SizedBox(height: 14),
                  _statCard("Correct Answers", user.totalCorrect.toString(),
                      Icons.verified_outlined),
                  const SizedBox(height: 14),
                  _statCard("Average Time / Question",
                      "${user.overallAvgTime}s", Icons.timer_outlined),
                  const SizedBox(height: 14),
                  _statCard("Weekly Improvement", "+${user.trend}%",
                      Icons.trending_up_rounded),
                ],
              ),
            ),

            const SizedBox(height: 30),
          ],
        ),
      ),
    );
  }

  // ======================= COMPONENT BUILDERS =======================

  Widget _sectionTitle(String text) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      child: Text(
        text,
        style: const TextStyle(
          fontSize: 30,
          fontWeight: FontWeight.bold,
          color: Colors.black87,
        ),
      ),
    );
  }

  Widget _statColumn(String label, String value) {
    return Column(
      children: [
        Text(
          value,
          style: const TextStyle(
            fontSize: 30,
            fontWeight: FontWeight.bold,
            color: Colors.white,
          ),
        ),
        const SizedBox(height: 4),
        Text(
          label,
          style: const TextStyle(
            fontSize: 20,
            color: Colors.white70,
          ),
        ),
      ],
    );
  }

  Widget _friendBubble(String name, String initial) {
    return Container(
      width: 70,
      margin: const EdgeInsets.only(right: 12),
      child: Column(
        children: [
          CircleAvatar(
            radius: 26,
            backgroundColor: Colors.grey.shade300,
            child: Text(
              initial,
              style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
            ),
          ),
          const SizedBox(height: 6),
          Text(name, style: const TextStyle(fontSize: 13)),
        ],
      ),
    );
  }

  void _showFriendsPopup(BuildContext context) {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: const Text("Your Friends"),
        content: SizedBox(
          width: 300,
          height: 300,
          child: ListView(
            children: const [
              ListTile(leading: CircleAvatar(child: Text("A")), title: Text("Ava")),
              ListTile(leading: CircleAvatar(child: Text("L")), title: Text("Leo")),
              ListTile(leading: CircleAvatar(child: Text("M")), title: Text("Maya")),
              ListTile(leading: CircleAvatar(child: Text("N")), title: Text("Noah")),
            ],
          ),
        ),
      ),
    );
  }

  Widget _statCard(String title, String value, IconData icon) {
    return Container(
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(
        color: Colors.grey.shade100,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: Colors.grey.shade300),
      ),
      child: Row(
        children: [
          Icon(icon, size: 30, color: Colors.redAccent),
          const SizedBox(width: 16),
          Expanded(
            child: Text(
              title,
              style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
            ),
          ),
          Text(
            value,
            style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
        ],
      ),
    );
  }

  Widget _collectionCard(String title, Color color) {
    return Container(
      width: 130,
      margin: const EdgeInsets.only(right: 16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(18),
        gradient: LinearGradient(
          colors: [color.withOpacity(0.7), color.withOpacity(0.4)],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        image: const DecorationImage(
          image: AssetImage("assets/placeholder_bg.jpg"),
          fit: BoxFit.cover,
          opacity: 0.15,
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Align(
          alignment: Alignment.bottomLeft,
          child: Text(
            title,
            style: const TextStyle(
              fontSize: 18,
              color: Colors.white,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),
      ),
    );
  }

  Widget _tag(String text) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        color: Colors.grey.shade200,
        borderRadius: BorderRadius.circular(18),
      ),
      child: Text(
        text,
        style: TextStyle(
          fontSize: 14,
          color: Colors.black.withOpacity(0.7),
        ),
      ),
    );
  }
}
