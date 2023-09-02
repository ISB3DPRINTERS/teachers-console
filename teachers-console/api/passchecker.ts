import * as fs from 'fs'
function contains(arr:any, key:any, val:any) {
    for (var i = 0; i < arr.length; i++) {
      if (arr[i][key] === val) return true;
    }
    return false;
  }
export default async function passchecker (username:any,password:any) {
    
    let teacherkeys:any= /* awaitfs.readFileSync('.../secretfiles/teacherkeys.txt'. 'utf8') */ {
        "teacher1": "password1",
    }
    if (contains(teacherkeys, username, password)==true) {
        return true
    }
    return false
}