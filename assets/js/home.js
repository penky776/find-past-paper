function submitForm() {
    const question = document.getElementById("user_input").value;
    const params = "user_input=" + question;
    const xhr = new XMLHttpRequest();
    xhr.open("POST", "http://localhost:3000/");
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
    xhr.send(params);
}