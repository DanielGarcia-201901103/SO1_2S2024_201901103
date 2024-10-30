from locust import HttpUser, task, between
import random

class APIUser(HttpUser):
    wait_time = between(1, 2)

    @task
    def send_data(self):
        # Elegir aleatoriamente entre Agronomía e Ingeniería
        faculty = random.choice(["Agronomía", "Ingeniería"])

        # Generar datos aleatorios para el estudiante
        student_name = f"Student {random.randint(1, 10000)}"
        age = random.randint(18, 30)
        discipline = random.randint(1, 3)

        # Selecciona la URL según la facultad
        if faculty == "Agronomía":
            # Endpoint Agronomía en el puerto 8080
            self.client.post(
                "http://localhost:8080/submitAgronomia",  # URL completa con puerto 8080
                json={
                    "student": student_name,
                    "age": age,
                    "faculty": faculty,
                    "discipline": discipline
                }
            )
        else:
            # Endpoint Ingeniería en el puerto 8081
            self.client.post(
                "http://localhost:8081/submitIngenieria",  # URL completa con puerto 8081
                json={
                    "student": student_name,
                    "age": age,
                    "faculty": faculty,
                    "discipline": discipline
                }
            )
