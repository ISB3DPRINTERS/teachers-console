import socketsender from "./middleware/sockets"
export const resethandler = (grade: number) => {
    console.log("reset"+grade)
    socketsender(grade)
}