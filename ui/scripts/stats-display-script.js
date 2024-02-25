var statDisplaySize = 100;
var statDisplayStart = [150,50];
draw(statDisplaySize, statDisplayStart);

// Original: points="150 50, 193 75, 193 125, 149 150, 105 125, 105 75, 150 50"
function draw(dist, startPoint) {
    let distance = dist;
    let angle = 60 * (Math.PI/180);

    let shape = document.querySelector("#base");
    let shapePoints = shape.getAttribute("points");
    
    // Start/HP point
    let pointsArr = [startPoint];
    placeLabel(startPoint[0],startPoint[1] - 10,"middle","HP");

    // ATK Point
    pointsArr.push([    newY(pointsArr[0][0], distance, angle),
                        newX(pointsArr[0][1], distance, angle)]);
    placeLabel(pointsArr[1][0]+10,pointsArr[1][1],"left","ATK");

    // DEF Point
    pointsArr.push([pointsArr[1][0], pointsArr[1][1]+distance]);
    placeLabel(pointsArr[2][0]+10,pointsArr[2][1],"left","DEF");

    // SPD Point
    pointsArr.push([    newY(pointsArr[2][0], distance, -angle),
                        newX(pointsArr[2][1], distance, -angle)]);
    placeLabel(pointsArr[3][0],pointsArr[3][1]+20,"middle","SPD");

    // SPC Point
    pointsArr.push([    newY(pointsArr[3][0], -distance, angle),
                        newX(pointsArr[3][1], -distance, angle)]);
    placeLabel(pointsArr[4][0]-30,pointsArr[4][1],"right","SPC");

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
    let angle = 60 * (Math.PI/180);

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
        // console.log("Value at " + points + ": " + pointsArr[points][0] + " " + pointsArr[points][1] + " ");
    
    //     let pointCircle = document.createElementNS("http://www.w3.org/2000/svg", "ellipse");
    //     pointCircle.setAttribute("cx", pointsArr[points][0]);
    //     pointCircle.setAttribute("cy", pointsArr[points][1]);
    //     pointCircle.setAttribute("rx", 5);
    //     pointCircle.setAttribute("ry", 5);
    //     pointCircle.setAttribute("id", points);
    //     document.getElementById("statWheel").appendChild(pointCircle);
    }
    shape.setAttribute("points", shapePoints);
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
    document.getElementById("statWheel").appendChild(textLabel);
}