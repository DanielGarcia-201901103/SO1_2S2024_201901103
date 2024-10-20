from locust import HttpUser, task, between

class APIUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_data(self):
        self.client.post("/submitAgronomia", json={
            "student": "Alvaro Garcia",
            "age": 20,
            "faculty": "Ingenieria",
            "discipline": 1
        })
