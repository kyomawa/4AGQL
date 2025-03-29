print("⚡ Script: Creating indexes for each database...");

// 1. AUTH DATABASE (auth-service)
db = db.getSiblingDB("auth-service");
db.getCollection("auth").createIndex(
  { user_id: 1 },
  { unique: true }
);
db.getCollection("auth").createIndex({ role: 1 });

// 2. USERS DATABASE (users-service)
db = db.getSiblingDB("users-service");
db.getCollection("users").createIndex(
  { email: 1 },
  { unique: true }
);
db.getCollection("users").createIndex(
  { pseudo: 1 },
  { unique: true }
);
db.getCollection("users").createIndex({ class_id: 1 });

// 3. GRADES DATABASE (grades-service)
db = db.getSiblingDB("grades-service");
db.getCollection("grades").createIndex({ user_id: 1 });
db.getCollection("grades").createIndex({ course: 1 });
db.getCollection("grades").createIndex({ user_id: 1, course: 1 });

// 4. CLASSES DATABASE (classes-service)
db = db.getSiblingDB("classes-service");
db.getCollection("classes").createIndex({ name: 1 }, { unique: true });