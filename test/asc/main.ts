@external("env", "alert")
declare function alert(s: string) : void
@external("env", "sendnumber")
declare function sendnumber(i: i32) : void

export function main() : void {
	alert("Hello world from AsseblyScript module!");
	sendnumber(69);
}

export function add(a: i32, b: i32) : i32 {
	main();
	return a + b;
}
