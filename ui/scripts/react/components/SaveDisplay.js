function GeneratedBoxes() {
    const boxes = [];

    for(let box = 0; box < 12; box++) {
        const images = []
        for(let row = 0; row < 4; row++) {
            const cols = []
            for(let col = 0; col < 5; col++) {
                cols.push(
                    <img className={`column-${col} boxCreature`} data-box={box} data-row={row} data-column={col}></img>
                )
            }
            images.push(
                <div className={`row-${row}`} style={{width: 'calc(45vw);'}}>
                    {
                        cols.map((item) => {
                            return item
                        })
                    }
                </div>
            )
        }

        boxes.push(
            <div className='pcBox' id={`box-${box}`} style={{display: 'none'}}>
                {
                    images.map((element) => {
                        return element;
                    })
                }
            </div>
        )
    }

    return (
        <>
            {
                boxes.map((box) => {
                    return box;
                })
            }
        </>
    )
}

export default function SaveDisplay() {
    return(
        <div id="saveDisplay">
            <div id="trainerDetails">
                <label for="trainerName">Trainer Name</label>
                <input  id="trainerName"    type="text" maxlength="11"/>
                <br/>
                <label for="trainerID">Trainer ID</label>
                <input  id="trainerID"      type="number" min="0" max="65535"/>
                <br/>
                <label for="trainerMoney">Trainer Money</label>
                <input  id="trainerMoney"   type="number"/>
            </div>
    
            <div id="party">
                <div className="creatureBox" id="1">
                    <button className="creatureSelect" id="1">
                        <img className="creature"/>
                    </button>
                </div>
                <div className="creatureBox" id="2">
                    <button className="creatureSelect" id="2">
                        <img className="creature"/>
                    </button>
                </div>
                <div className="creatureBox" id="3">
                    <button className="creatureSelect" id="3">
                        <img className="creature"/>
                    </button>
                </div>
                <div className="creatureBox" id="4">
                    <button className="creatureSelect" id="4">
                        <img className="creature"/>
                    </button>
                </div>
                <div className="creatureBox" id="5">
                    <button className="creatureSelect" id="5">
                        <img className="creature"/>
                    </button>
                </div>
                <div className="creatureBox" id="6">
                    <button className="creatureSelect"  id="6">
                        <img className="creature"/>
                    </button>
                </div>
            </div>

            <div id="boxes" style={{display: 'none'}}>
                <h3>PC Pokemon</h3>
                <div className="boxDisplay">
                    <div id="boxControls">
                        <input type="button" id="backBoxBtn" className="boxBtn" value="<"/>
                        <div id="pcBoxNum"></div>
                        <input type="button" id="frwdBoxBtn" className="boxBtn" value=">"/>
                    </div>

                    {/* <div className="pcBox" id="box-0" style={{display: 'none'}}>
                            <div className="row-0" style={{width: 'calc(45vw);'}}>
                                <img className="column-0 boxCreature" data-box="0" data-row="0" data-column="0"></img>
                                <img className="column-1 boxCreature" data-box="0" data-row="0" data-column="1"></img>
                                <img className="column-2 boxCreature" data-box="0" data-row="0" data-column="2"></img>
                                <img className="column-3 boxCreature" data-box="0" data-row="0" data-column="3"></img>
                                <img className="column-4 boxCreature" data-box="0" data-row="0" data-column="4"></img>
                            </div>
                            <div className="row-1" style={{width: 'calc(45vw);'}}>
                                <img className="column-0 boxCreature" data-box="0" data-row="1" data-column="0"></img>
                                <img className="column-1 boxCreature" data-box="0" data-row="1" data-column="1"></img>
                                <img className="column-2 boxCreature" data-box="0" data-row="1" data-column="2"></img>
                                <img className="column-3 boxCreature" data-box="0" data-row="1" data-column="3"></img>
                                <img className="column-4 boxCreature" data-box="0" data-row="1" data-column="4"></img>
                            </div>
                            <div className="row-2" style={{width: 'calc(45vw);'}}>
                                <img className="column-0 boxCreature" data-box="0" data-row="2" data-column="0"></img>
                                <img className="column-1 boxCreature" data-box="0" data-row="2" data-column="1"></img>
                                <img className="column-2 boxCreature" data-box="0" data-row="2" data-column="2"></img>
                                <img className="column-3 boxCreature" data-box="0" data-row="2" data-column="3"></img>
                                <img className="column-4 boxCreature" data-box="0" data-row="2" data-column="4"></img>
                            </div>
                            <div className="row-3" style={{width: 'calc(45vw);'}}>
                                <img className="column-0 boxCreature" data-box="0" data-row="3" data-column="0"></img>
                                <img className="column-1 boxCreature" data-box="0" data-row="3" data-column="1"></img>
                                <img className="column-2 boxCreature" data-box="0" data-row="3" data-column="2"></img>
                                <img className="column-3 boxCreature" data-box="0" data-row="3" data-column="3"></img>
                                <img className="column-4 boxCreature" data-box="0" data-row="3" data-column="4"></img>
                            </div>
                    </div> */}
                    <GeneratedBoxes/>
 
                </div>
             </div>

        </div>
    )
}