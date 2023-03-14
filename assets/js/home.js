function submitForm() {
    const subject = document.getElementById("subject").value;
    const question = document.getElementById("user_input").value;
    const params = "user_input=" + question + "&subject=" + subject;
    const xhr = new XMLHttpRequest();
    xhr.open("POST", "http://localhost:3000/");
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
    xhr.send(params);
    document.getElementById('output').innerHTML = "loading...";

    xhr.onload = () => {
        if (xhr.readyState === xhr.DONE && xhr.status === 200) {
            var response = xhr.response;
            if (response === "") {
                var response = "No matches found."
            };
            document.getElementById('output').innerHTML = makeBold(response, question);
        }
    };
}

function makeBold(input, wordToBold) {
    return input.replace(new RegExp(wordToBold, "ig"), '<b>' + wordToBold + '</b>');
}