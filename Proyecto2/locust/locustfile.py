from locust import HttpUser, task, between

class APIUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_data(self):
        # En discipline puede ser 1,2,3
        self.client.post("/submitAgronomia", json={
            "student": "María de los angeles",
            "age": 20,
            "faculty": "Agronomía",
            "discipline": 1
        })

        self.client.post("/submitIngenieria", json={
            "student": "Juan perez",
            "age": 20,
            "faculty": "Ingeniería",
            "discipline": 1
        })
