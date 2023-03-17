function submitForm() {
    var subject = document.getElementById("subject").value;
    var question = document.getElementById("user_input").value;
    var params = "user_input=" + question + "&subject=" + subject;
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "http://localhost:3000/");
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
    xhr.send(params);
    load();
    xhr.onload = function () {
        if (xhr.readyState === xhr.DONE && xhr.status === 200) {
            var response = xhr.response;
            if (response === "") {
                var response = "No matches found.";
            };
            document.getElementById('output').innerHTML = response;
        }
    };
}

function load() {
    var root = ReactDOM.createRoot(document.getElementById('output'));
    var element = React.createElement(
        "div",
        { "className": "lds-spinner" },
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null),
        React.createElement("div", null)
    );
    root.render(element);
}