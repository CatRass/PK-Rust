var statDisplaySize = 100;
var statDisplayStart = [150,50];
draw(statDisplaySize, statDisplayStart);

// Original: points="150 50, 193 75, 193 125, 149 150, 105 125, 105 75, 150 50"
function draw(dist, startPoint) {
    let distance = dist;
    let angle = 60 * (Math.PI/180);

    let shape = document.querySelector("#base");
    let shapePoints = shape.getAttribute("points");

    let labelGroup = document.createElementNS("http://www.w3.org/2000/svg","g")
    labelGroup.setAttribute("id","labelGroup");
    document.getElementById("statWheel").appendChild(labelGroup)

    let inputGroup = document.createElementNS("http://www.w3.org/2000/svg","g")
    inputGroup.setAttribute("id","inputGroup");
    document.getElementById("statWheel").appendChild(inputGroup)
    
    // Start/HP point
    let pointsArr = [startPoint];
    placeLabel(startPoint[0],startPoint[1] - 10,"middle","HP");
    placeInput(startPoint[0],startPoint[1] - 50, "hpInput");

    // ATK Point
    pointsArr.push([    newY(pointsArr[0][0], distance, angle),
                        newX(pointsArr[0][1], distance, angle)]);
    placeLabel(pointsArr[1][0]+10,pointsArr[1][1],"left","ATK");
    placeInput(pointsArr[1][0]+10,pointsArr[1][1] + 10, "atkInput");

    // DEF Point
    pointsArr.push([pointsArr[1][0], pointsArr[1][1]+distance]);
    placeLabel(pointsArr[2][0]+10,pointsArr[2][1],"left","DEF");
    placeInput(pointsArr[2][0]+10,pointsArr[2][1] + 10, "defInput");

    // SPD Point
    pointsArr.push([    newY(pointsArr[2][0], distance, -angle),
                        newX(pointsArr[2][1], distance, -angle)]);
    placeLabel(pointsArr[3][0],pointsArr[3][1]+20,"middle","SPD");
    placeInput(pointsArr[3][0],pointsArr[3][1]+30, "spdInput");

    // SPC Point
    pointsArr.push([    newY(pointsArr[3][0], -distance, angle),
                        newX(pointsArr[3][1], -distance, angle)]);
    placeLabel(pointsArr[4][0]-30,pointsArr[4][1],"right","SPC");
    placeInput(pointsArr[4][0]-30,pointsArr[4][1] + 10, "spcInput");

    // Not used in Gen 1/2
    pointsArr.push([pointsArr[4][0], pointsArr[4][1]-distance]);
    pointsArr.push([pointsArr[0][0],pointsArr[0][1]]);

    for (let points in pointsArr) {
        shapePoints += pointsArr[points][0] + " " + pointsArr[points][1] + " ";
    }

    shape.setAttribute("points", shapePoints);

}

export function drawStats(pkmnStats) {
    let midPoint = [statDisplayStart[0], statDisplayStart[1]+statDisplaySize];
    let singleIV = statDisplaySize/15;

    let shape = document.querySelector("#stats");
    let shapePoints = shape.getAttribute("points");
    shapePoints = "";

    let pointsArr = [midPoint];
    
    pointsArr.push(
        [
            midPoint[0],
            midPoint[1]-((pkmnStats.hp)*singleIV)
        ]
    );

    pointsArr.push(
        [
            newX(midPoint[0], (pkmnStats.atk)*singleIV, (30 * (Math.PI/180))),
            newY(midPoint[1], -((pkmnStats.atk)*singleIV),(30  * (Math.PI/180)))
        ]
    );

    pointsArr.push(
        [
            newX(midPoint[0], (pkmnStats.def)*singleIV, -(330 * (Math.PI/180))),
            newY(midPoint[1], (pkmnStats.def)*singleIV, -(330 * (Math.PI/180)))
        ]
    );

    pointsArr.push(
        [
            midPoint[0],
            newY(midPoint[1],-(pkmnStats.spd)*singleIV, 270* (Math.PI/180))
        ]
    );

    pointsArr.push(
        [
            newX(midPoint[0], ((pkmnStats.spc)*singleIV), (210 * (Math.PI/180))),
            newY(midPoint[1], -(pkmnStats.spc)*singleIV, (210 * (Math.PI/180)))
        ]
    );

    pointsArr.push([pointsArr[0][0],pointsArr[0][1]]);

    for (let points in pointsArr) {
        shapePoints += pointsArr[points][0] + "," + pointsArr[points][1] + " ";
    }
    shape.setAttribute("points", shapePoints);
    setInputValues("hpInput",pkmnStats.hp);
    setInputValues("defInput",pkmnStats.def);
    setInputValues("atkInput",pkmnStats.atk);
    setInputValues("spdInput",pkmnStats.spd);
    setInputValues("spcInput",pkmnStats.spc);
}

function newX(lastX, dist, angle) {
    return lastX + (dist * Math.cos(angle));
}

function newY(lastY, dist, angle) {
    return lastY + (dist * Math.sin(angle));
}

function placeLabel(x,y,anchor,text) {
    let textLabel = document.createElementNS("http://www.w3.org/2000/svg", "text");
    textLabel.setAttribute("x", x);
    textLabel.setAttribute("y", y);
    textLabel.setAttribute("fill", "black");
    textLabel.setAttribute("text-anchor",anchor)
    textLabel.innerHTML = text;
    document.getElementById("labelGroup").appendChild(textLabel);
}

function placeInput(x,y,id) {
    let inputObject = document.createElementNS("http://www.w3.org/2000/svg","foreignObject");
    inputObject.setAttribute("class","ivInputSVG");
    inputObject.setAttribute("x", x);
    inputObject.setAttribute("y", y);
    inputObject.setAttribute("width",100)
    inputObject.setAttribute("height",50)
    let input = document.createElementNS("http://www.w3.org/1999/xhtml", "input");
    input.setAttribute("xmlns", "http://www.w3.org/1999/xhtml");
    input.setAttribute("type", "number");
    input.setAttribute("min", 0);
    input.setAttribute("max", 15);
    // input.setAttribute("readonly", true);
    input.setAttribute("id", id);
    input.setAttribute("class", "ivInput");
    input.setAttribute("style", "width: 50px;");

    inputObject.appendChild(input);
    document.getElementById("inputGroup").appendChild(inputObject);
    
}

function setInputValues(id, value) {
    document.getElementById(id).setAttribute("value", value);
    document.getElementById(id).setAttribute("data-value", value);
}