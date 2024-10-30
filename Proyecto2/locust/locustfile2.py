from locust import HttpUser, task, between
import random

class APIUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_data(self):
        # Randomly choose between Agronomia and Ingenieria
        faculty = random.choice(["Agronomía", "Ingeniería"])

        # Generate random data for the student
        student_name = f"Student {random.randint(1, 10000)}"
        age = random.randint(18, 30)
        discipline = random.randint(1, 3)

        if faculty == "Agronomía":
            self.client.post("/submitAgronomia", json={
                "student": student_name,
                "age": age,
                "faculty": faculty,
                "discipline": discipline
            })
        else:
            self.client.post("/submitIngenieria", json={
                "student": student_name,
                "age": age,
                "faculty": faculty,
                "discipline": discipline
            })
