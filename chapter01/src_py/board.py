# -*- coding: utf-8 -*-

try:
    from typing import Iterable, List  # NOQA: F401  pylint: disable=unused-import
except ImportError:
    pass


def main():
    # type: () -> None
    for i in create_records():
        for j in show_board(i):
            print(j)


def create_records():
    # type: () -> Iterable[List[List[int]]]
    yield [[0, 0, 0], [0, 1, 0], [0, 0, 0], ]
    yield [[0, 2, 0], [0, 1, 0], [0, 0, 0], ]
    yield [[0, 2, 0], [0, 1, 0], [1, 0, 0], ]
    yield [[0, 2, 2], [0, 1, 0], [1, 0, 0], ]
    yield [[1, 2, 2], [0, 1, 0], [1, 0, 0], ]
    yield [[1, 2, 2], [2, 1, 0], [1, 0, 0], ]
    yield [[1, 2, 2], [2, 1, 0], [1, 0, 1], ]


def show_board(board):
    # type: (List[List[int]]) -> Iterable[str]
    separator = "+---+---+---+"
    yield ""
    yield separator
    for row in board:
        yield "| {0} |".format(" | ".join(from_int_to_str(i) for i in row))
        yield separator
    yield ""


def from_int_to_str(x):
    if x == 0:
        return " "
    if x == 1:
        return "O"
    if x == 2:
        return "X"



#def create_table(data, dom_id, title, marubatuflag, check_line_flag):
#    # table = document.createElement("table");
#    if(title){
#        let caption = document.createElement("caption");
#        caption.appendChild( document.createTextNode( title ));
#        table.appendChild(caption);
#    }
#    table.id = id_num;
#    table.classList.add("record");
#    let results = checkLine( data );
#
#    # 行
#    for(let i = 0; i < data.length; i++){
#        let row =  table.insertRow(-1);
#        //列
#        for(let j = 0; j < data[0].length; j++){
#            let td = row.insertCell(-1);
#            td.id = id + "_" + i + j;
#
#            let text = "";
#            if( marubatuflag ){
#                if( data[i][j] ==1) text = "○";
#                else if(data[i][j]==2) text = "×";
#            }else{
#                text = data[i][j];
#            }
#
#            if(results[i][j] == 1 && checkLineflag) td.classList.add("winMaru");
#            if(results[i][j] == -1 && checkLineflag) td.classList.add("winBatu");
#
#            td.appendChild( document.createTextNode( text ) );
#        }
#    }
#
#
#    if(domID){
#        document.querySelector("#" + domID).appendChild(table);
#
#    }else{
#        document.querySelectorAll("body")[0].appendChild(table);
#    }
#
#
#
#function checkLine( data ){
#    let results = [];
#    for(let i = 0; i < data.length; i++){
#        results[i] = [];
#        for(let j = 0; j < data[i].length; j++){
#            results[i][j] = 0;
#        }
#    }
#
#    //横列
#    for( let i = 0; i<data.length; i++ ){
#        if( data[i][0] * data[i][1] * data[i][2] == 1 ) {
#            for( let j = 0; j < data[0].length; j++ ) results[i][j] = 1;
#        }
#        if( data[i][0] * data[i][1] * data[i][2] == 8 ){
#            for( let j = 0; j < data[0].length; j++ ) results[i][j] = -1;
#        }
#    }
#    //縦列
#    for( let j = 0; j<data[0].length; j++ ){
#        if( data[0][j] * data[1][j] * data[2][j] == 1 ) {
#            for( let i = 0; i < data.length; i++ ) results[i][j] = 1;
#        }
#        if( data[0][j] * data[1][j] * data[2][j] == 8 ) {
#            for( let i = 0; i < data.length; i++ ) results[i][j] = -1;
#        }
#    }
#
#    //右上斜め
#    if( data[0][0] * data[1][1] * data[2][2] == 1 ) {
#        for( let i = 0; i<data.length; i++ ) results[i][i] = 1;
#    }
#    if( data[0][0] * data[1][1] * data[2][2] == 8 ) {
#        for( let i = 0; i<data.length; i++ ) results[i][i] = -1;
#    }
#    //右下斜め
#    if( data[2][0] * data[1][1] * data[0][2] == 1 ) {
#        for( let i = 0; i<data.length; i++ ) results[data.length-1-i][i] = 1;
#    }
#    if( data[2][0] * data[1][1] * data[0][2] == 8 ) {
#        for( let i = 0; i<data.length; i++ ) results[data.length-1-i][i] = -1;
#    }
#
#    return results;
#}
#
#
#
#//対称反転
#function mirrorSymmetry(source, r ){
#    let Nm = source.length - 1;
#
#    let results = [];
#    for(let i = 0; i <= Nm; i++){
#        results[i] = [];
#    }
#    r = r || 0;
#    if( r == 0 ){
#        for(let i = 0; i <= Nm; i++){
#            for(let j = 0; j <= Nm; j++){
#                results[i][j] = source[i][j];
#            }
#        }
#    }
#    //横軸
#    if( r == 1 ){
#        /*
#        results[0][0] = source[2][0];
#        results[0][1] = source[2][1];
#        results[0][2] = source[2][2];
#
#        results[1][0] = source[1][0];
#        results[1][1] = source[1][1];
#        results[1][2] = source[1][2];
#
#        results[2][0] = source[0][0];
#        results[2][1] = source[0][1];
#        results[2][2] = source[0][2];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[Nm-i][j];
#            }
#        }
#
#    }
#    //縦軸
#    if( r == 2 ){
#        /*
#        results[0][0] = source[0][2];
#        results[0][1] = source[0][1];
#        results[0][2] = source[0][0];
#
#        results[1][0] = source[1][2];
#        results[1][1] = source[1][1];
#        results[1][2] = source[1][0];
#
#        results[2][0] = source[2][2];
#        results[2][1] = source[2][1];
#        results[2][2] = source[2][0];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[i][Nm-j];
#            }
#        }
#    }
#
#    //右上斜軸
#    if( r == 3 ){
#        /*
#        results[0][0] = source[2][2];
#        results[0][1] = source[1][2];
#        results[0][2] = source[0][2];
#        results[1][0] = source[2][1];
#        results[1][1] = source[1][1];
#        results[1][2] = source[0][1];
#        results[2][0] = source[2][0];
#        results[2][1] = source[1][0];
#        results[2][2] = source[0][0];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[Nm-j][Nm-i];
#            }
#        }
#    }
#    //右下斜軸
#    if( r == 4 ){
#        /*
#        results[0][0] = source[0][0];
#        results[0][1] = source[1][0];
#        results[0][2] = source[2][0];
#        results[1][0] = source[0][1];
#        results[1][1] = source[1][1];
#        results[1][2] = source[2][1];
#        results[2][0] = source[0][2];
#        results[2][1] = source[1][2];
#        results[2][2] = source[2][2];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[j][i];
#            }
#        }
#    }
#    return results;
#
#}
#//反時計回り
#function rotationSymmetry( source, r ){
#    let Nm = source.length - 1;
#
#    let results = [];
#    for(let i = 0; i <= Nm; i++){
#        results[i] = [];
#    }
#    r = r || 0;
#    if( r == 0 ){
#        for(let i = 0; i <= Nm; i++){
#            for(let j = 0; j <= Nm; j++){
#                results[i][j] = source[i][j];
#            }
#        }
#    }
#    //90°回転
#    if( r == 1 ){
#        /*
#        results[0][0] = source[0][2];
#        results[0][1] = source[1][2];
#        results[0][2] = source[2][2];
#
#        results[1][0] = source[0][1];
#        results[1][1] = source[1][1];
#        results[1][2] = source[2][1];
#
#        results[2][0] = source[0][0];
#        results[2][1] = source[1][0];
#        results[2][2] = source[2][0];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[j][Nm-i];
#            }
#        }
#    }
#    //180°回転
#    if( r == 2 ){
#        /*
#        results[0][0] = source[2][2]; //9
#        results[0][1] = source[2][1]; //8
#        results[0][2] = source[2][0]; //7
#
#        results[1][0] = source[1][2]; //6
#        results[1][1] = source[1][1]; //5
#        results[1][2] = source[1][0]; //4
#
#        results[2][0] = source[0][2]; //3
#        results[2][1] = source[0][1]; //2
#        results[2][2] = source[0][0]; //1
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[Nm-i][Nm-j];
#            }
#        }
#    }
#    //270°回転
#    if( r == 3 ){
#        /*
#        results[0][0] = source[2][0];
#        results[0][1] = source[1][0];
#        results[0][2] = source[0][0];
#
#        results[1][0] = source[2][1];
#        results[1][1] = source[1][1];
#        results[1][2] = source[0][1];
#
#        results[2][0] = source[2][2];
#        results[2][1] = source[1][2];
#        results[2][2] = source[0][2];
#        */
#        for(let i=0; i<= Nm; i++){
#            for(let j=0; j<= Nm; j++){
#                results[i][j] = source[Nm-j][i];
#            }
#        }
#    }
#
#    return results;
#}
#//点対称
#function pointSymmetry( source ){
#    let results = [];
#    for(let i = 0; i < source.length; i++){
#        results[i] = [];
#    }
#
#    results[0][0] = source[2][2]; //8
#    results[0][1] = source[2][1]; //7
#    results[0][2] = source[2][0]; //6
#
#    results[1][0] = source[1][2]; //5
#    results[1][1] = source[1][1]; //4
#    results[1][2] = source[1][0]; //3
#
#    results[2][0] = source[0][2]; //2
#    results[2][1] = source[0][1]; //1
#    results[2][2] = source[0][0]; //0
#
#    return results;
#
#}
#//状態値を計算する関数
#function getStateValue(record, baseValue){
#    let v = 0;
#    for( let i = 0; i < record.length; i++ ){
#        for( let j = 0; j < record[ 0 ].length; j++ ){
#            v += record[i][j] * baseValue[i][j];
#        }
#    }
#    return v;
#}
#//状態値が最小値となる対称性と状態値を計算
#function getMinValue( record, baseValue ){
#    let min_v = Math.pow(3,10);
#    let min_r = 0;
#    let min_m = 0;
#    for( let r = 0; r <=3; r++  ){
#        for( let m = 0; m <=4; m++  ){
#            let _record = rotationSymmetry( record, r );
#            let __record = mirrorSymmetry( _record, m );
#            let v = getStateValue(__record, baseValue);
#            //より小さい状態値であれば更新
#            if ( v < min_v ){
#                min_v = v;
#                min_r = r;
#                min_m = m;
#            }
#        }
#    }
#    return {value :min_v, rotationSymmetry : min_r, mirrorSymmetry : min_m };
#}


if __name__ == '__main__':
    main()
