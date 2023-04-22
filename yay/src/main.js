const { invoke } = window.__TAURI__.tauri;


let greetInputEl;
let greetMsgEl;
let jsonContent;
//create an async function for get_system_info

async function get_system_info() {
  //invoke the get_system_info command
  const result = await invoke("get_system_info");
  //check if result is not null or undefined
  if (result) {
    //log the result
    const parsedResult = JSON.parse(result);
    console.log(parsedResult);
    
    //take the result and change the system-info div in welcome.html but outline the html of each <p> tag
    document.getElementById("sys-name").innerHTML = parsedResult.system_name;
    document.getElementById("sys-kernel").innerHTML = parsedResult.kernel_version;
    document.getElementById("os-version").innerHTML = parsedResult.os_version;
    document.getElementById("host-name").innerHTML = parsedResult.hostname;
  } else {
    console.log("Result is null or undefined");
  }
}

// //create an async function for get_system_usage
// async function get_system_usage() {
//   //invoke the get_system_usage command every 5 seconds
//   const result = await invoke("get_system_usage");
//   const parsedResult = JSON.parse(result);

//   //update memory-bar, cpu-bar, and disk-bar with the new values but represent it using a progress bar

// }

async function omarlog(message) {
  await invoke("log_to_terminal", { message });
}

// async function get_system_usage() {
//   const result = await invoke("get_system_usage");
//   const parsedResult = JSON.parse(result);
//   omarlog(parsedResult.mem_percent);
//   omarlog("oh no");

//   const memoryBar = document.querySelector('.memory-bar .bar');
//   const cpuBar = document.querySelector('.cpu-bar .bar');
//   const diskBar = document.querySelector('.swap-bar .bar');

//   document.getElementById("#bar-label-1").innerHTML = parsedResult.mem_percent;
//   document.getElementById("#bar-label-2").innerHTML = parsedResult.cpu_percent;
//   document.getElementById("#bar-label-3").innerHTML = parsedResult.swap_percent;

//   memoryBar.style.width = parsedResult.mem_percent.toFixed(1) + '%';
//   cpuBar.style.width = parsedResult.cpu_percent.toFixed(1) + '%';
//   diskBar.style.width = parsedResult.swap_percent.toFixed(1) + '%';
// }

async function updateProgressBars() {
  const usage = await invoke("get_system_usage");
  const parsedResult = JSON.parse(usage);

  const memoryBar = document.getElementById("memory-bar");
  memoryBar.style.width = `${parsedResult.mem_percent}%`;

  const cpuBar = document.getElementById("cpu-bar");
  cpuBar.style.width = `${parsedResult.cpu_percent}%`;
  omarlog(cpuBar.style.width);

  const swapBar = document.getElementById("swap-bar");
  swapBar.style.width = `${parsedResult.swap_percent}%`;

  const memoryBarPercentage = document.getElementById("memory-bar-percentage");
  memoryBarPercentage.textContent = `${parsedResult.mem_percent}%`;

  const cpuBarPercentage = document.getElementById("cpu-bar-percentage");
  cpuBarPercentage.textContent = `${parsedResult.cpu_percent}%`;

  const swapBarPercentage = document.getElementById("swap-bar-percentage");
  swapBarPercentage.textContent = `${parsedResult.swap_percent}%`;

}

setInterval(updateProgressBars, 2500);

//create an event listener for get_system_info to run when the page loads but doens't close and open the application multiple times
document.addEventListener("DOMContentLoaded", function() {
  console.log("inmainjs");
 get_system_info();
  //get_system_usage();
}
);
