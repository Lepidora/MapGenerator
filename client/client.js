/*
 *  client.js
 *  
 *  Scripting for the client page
 */

var worldid = 0;

function checkURL() {

    let path = window.location.pathname;

    if (path === "/") {

        getWorldID();

    } else {

        let split_path = path.split('/');

        if (split_path.length > 0) {

            let id = split_path[split_path.length - 1];

            worldid = id;

            getWorldData(id);

            setupMap();
        }
    }
}

function newWorldClick() {

    getWorldID();
}

function getWorldID() {

    let seed = document.getElementById('seed').value;
    let name = document.getElementById('name').value;
    let sealevel = document.getElementById('sealevel').value;
    let temperature = document.getElementById('temperature').value;
    let humidity = document.getElementById('humidity').value;

    console.log("Seed: " + seed + " Name: " + name + " Sea level: " + sealevel + " Temperature: " + temperature + " Humidity: " + humidity);

    let body = {
        seed: seed,
        name: name,
        sea_level: sealevel,
        temperature: temperature,
        humidity: humidity
    };

    doPostRequest('new', body, function (err, event) {

        if (err) {
            console.log('Error making new world: ' + err);
            alert('Error making new world: ' + err);
        } else {

            let request = event.target;

            if (request.readyState === 4 && request.status === 200) {

                //let results = JSON.parse(request.responseText);

                console.log(request.responseText);

                let response = JSON.parse(request.responseText);

                let id = response.id;

                if (id !== undefined) {

                    worldid = id;

                    window.history.pushState(id, 'World ' + id, '/' + id);

                    setParameters(response);

                    setupMap();
                }
            }
        }
    });
}

function getWorldData(id) {

    doGetRequest('get/' + id, undefined, function (err, event) {

        if (err) {
            console.log('Error fetching world data: ' + err);
            alert('Error fetching world data: ' + JSON.stringify(err));
        } else {

            let request = event.target;

            if (request.readyState === 4 && request.status === 200) {

                console.log(request.responseText);

                let response = JSON.parse(request.responseText);

                setParameters(response);
            }
        }
    });
}

function setParameters(response) {

    if (response.seed !== undefined) {
        document.getElementById('seed').value = response.seed;
    }

    if (response.name !== undefined) {
        document.getElementById('name').value = response.name;
    }

    let sea_level = response.sea_level;

    if (sea_level !== undefined && !isNaN(sea_level)) {
        document.getElementById('sealevel').value = response.sea_level;
    }

    let temperature = response.temperature;

    if (temperature !== undefined && !isNaN(temperature)) {
        document.getElementById('temperature').value = temperature;
    }

    let humidity = response.humidity;

    if (humidity !== undefined && !isNaN(humidity)) {
        document.getElementById('humidity').value = humidity;
    }
}

function setupMap() {

    var minZoom = 2;
    var maxZoom = 8;

    var projection = new ol.proj.Projection({

        code: 'EPSG:21781',
        units: 'm',
        //worldExtent: [-179, -89.99, 179, 89.99]
    });

    var map = new ol.Map({
        loadTilesWhileAnimating: true,
        loadTilesWhileInteracting: true,
        target: 'map',
        layers: [
            new ol.layer.Tile({
                //source: new ol.source.OSM()
                source: new ol.source.XYZ({
                    //projection: projection,
                    url: "http://localhost:8080/tiles/" + worldid + "/{z}/{x}/{y}",
                    //url: "https://{a-c}.tile.openstreetmap.org/{z}/{x}/{y}.png",
                    tileSize: [256, 256],
                    minZoom: minZoom,
                    maxZoom: maxZoom,
                    wrapX: false
                })
            })
        ],
        view: new ol.View({
            center: ol.proj.fromLonLat([0, 0]),
            maxResolution: 40075016.68557849 / 256 / Math.pow(2, minZoom),//1,
            minResolution: 40075016.68557849 / 256 / Math.pow(2, maxZoom),//1 / Math.pow(2, 28),
            zoom: 0
        })
    });
}