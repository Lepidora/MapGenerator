/*
 *  common.js
 *  
 *  Reused common JavaScript functions
 */

function doGetRequest(endpoint, parameters, callback) {

    var request = new XMLHttpRequest();
    var url = endpoint;

    if (parameters) {

        url += '?';

        for (var i = 0; i < parameters.length; i++) {

            var parameter = parameters[i];

            if (i > 0) {
                url += '&';
            }

            url += parameter;
        }
    }

    request.onreadystatechange = function (request) {
        callback(undefined, request);
    }

    request.open('GET', url, true);
    request.send();
}

function doPostRequest(endpoint, body, callback) {

    try {

        var request = new XMLHttpRequest();

        request.onreadystatechange = function (event) {
            callback(undefined, event);
        }

        request.open('POST', endpoint, true);

        request.setRequestHeader("Content-type", "application/json");

        request.send(JSON.stringify(body));

    } catch (error) {
        callback(error);
    }
}

function clearChildren(node) {

    while (node.firstChild) {
        node.removeChild(node.firstChild);
    }
}