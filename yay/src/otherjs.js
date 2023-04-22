//take json output returned from get_system_info and display it in the system-info div in welcome.html
// $.ajax({
//   url: "system-info.json",
//   dataType: "json",
//   success: function(data) {
//     console.log(data);

//     //change html of element sys-name
//     $("#sys-name").html(data.system_name);
//     $("#sys-kernel").html(data.kernel_version);
//     $("#os-version").html(data.os_version);
//     $("#host-name").html(data.hostname);
//   },
//   error: function() {
//     console.log("error");
//   }
// });

// document.addEventListener("DOMContentLoaded", function() {
//   get_JSON_contents();
// });
