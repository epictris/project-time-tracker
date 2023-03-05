// Time Processing

export function showHours(millis : number) : boolean {
  return getHours(millis) > 0;
}

export function showMinutes(millis : number) : boolean {
  return getMinutes(millis) > 0 && millis < 60 * 60 * 10 * 1000;
}

export function showSeconds(millis : number) : boolean {
  return millis < 10 * 60 * 1000 && (getSeconds(millis) > 0 || (millis < 1000 && millis > 0));
}

export function getHours(millis : number) : number {
  return Math.floor(millis / (1000 * 60 * 60))
}

export function getMinutes(millis : number) : number {
  return Math.floor(millis/(1000 * 60)) % 60;
}

export function getSeconds(millis : number) : number {
  return Math.floor(millis/1000) % 60;
}

// Date Processing

export function UTCStringToMillis(UTCString : string | null) {
  if(UTCString) return new Date(UTCString + " UTC").getTime()
  return null;
}

export function getDate(UTCString : string) {
  let dateTime = new Date(UTCStringToMillis(UTCString)!)
  return ("0" + dateTime.getDate()).slice(-2);
}

export function getMonth(UTCString : string) {
  let dateTime = new Date(UTCStringToMillis(UTCString)!)
  return dateTime.toDateString().slice(3, 7);
}

export function getYear(UTCString : string) {
  let dateTime = new Date(UTCStringToMillis(UTCString)!)
  return dateTime.getFullYear().toString().slice(2, 4);
}