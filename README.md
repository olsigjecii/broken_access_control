# Broken Access Control in Rust 🦀

## Step 1: Run the Server

From your terminal, inside the rust_boken_access_control directory, run the application:

```bash
cargo run
```

You should see the server start up and print a confirmation message:
🚀 Server starting at http://127.0.0.1:8080

Vulnerable endpoints: /grades
Secure endpoints: /secure/grades

## Step 2 : Demo the Attack on Vulnerable Endpoints

Open a new terminal window to perform the following tests.

```bash
# 1. View another student's grade (IDOR Attack)
# You are logged in as Ezra (ID 20223948), but you ask for the grade of student 20223949. The vulnerable endpoint allows this.

curl 'http://127.0.0.1:8080/grades?studentid=20223949&subjectid=1293'

# # {"grade":"A+"}
# This shows you successfully viewed data that is not yours.

# 2. Update your own grade (Missing Role Check Attack)
# As a student, you should not be able to change your grade. But the vulnerable PATCH endpoint has no role check.

curl -X PATCH 'http://127.0.0.1:8080/grades' \
-H "Content-Type: application/json" \
-d '{"studentid":"20223948", "subjectid":"1293", "grade":"A++"}'

# {"message":"Grade updated successfully","new_grade":"A++"}
```

## Step 3. Verify the malicious update worked

Now, check your own grade again to confirm the attack was successful.

```bash
curl 'http://127.0.0.1:8080/grades?studentid=20223948&subjectid=1293'

# {"grade":"A++"}
```

## Step 4: Demo the Fix on Secure Endpoints

Now, run the same attacks against the /secure endpoints to show how the fixes work.

1. Attempt to view another student's grade (Blocked)
   This time, the access control logic will kick in and deny the request. The -i flag is added to show the HTTP status code.

```bash
curl -i 'http://127.0.0.1:8080/secure/grades?studentid=20223949&subjectid=1293'

# Expected Output:
# You will see a 403 Forbidden status and the access denied message.

# HTTP/1.1 403 Forbidden
# content-length: 49
# content-type: text/plain; charset=utf-8
# date: Wed, 11 Jun 2025 08:29:51 GMT

# Access Denied: You can only view your own grades.

# 2. Attempt to update a grade as a student (Blocked)
# The role-based check will now prevent you from making unauthorized changes.


curl -i -X PATCH 'http://127.0.0.1:8080/secure/grades' \
-H "Content-Type: application/json" \
-d '{"studentid":"20223948", "subjectid":"1293", "grade":"HACKED"}'

# You will see another 403 Forbidden status, proving the role check is working.


# HTTP/1.1 403 Forbidden
# content-length: 56
# content-type: text/plain; charset=utf-8
# date: Wed, 11 Jun 2025 08:29:51 GMT

# Access Denied: You are not authorized to update grades.
```

**This demo clearly illustrates the vulnerability and confirms that the server-side authorization checks effectively mitigate it.**

## Run Client (enumerate)

```bash
# This command specifically runs the 'client' binary
cargo run --bin client
```
