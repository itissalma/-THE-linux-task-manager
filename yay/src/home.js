var parallax = function(e) {
    var windowWidth = $(window).width();
    if (windowWidth < 768) return;
    var halfFieldWidth = $(".parallax").width() / 2,
      halfFieldHeight = $(".parallax").height() / 2,
      fieldPos = $(".parallax").offset(),
      x = e.pageX,
      y = e.pageY - fieldPos.top,
      newX = (x - halfFieldWidth) / 30,
      newY = (y - halfFieldHeight) / 30;
    $('.parallax [class*="wave"]').each(function(index) {
      $(this).css({
        transition: "",
        transform:
          "translate3d(" + index * newX + "px," + index * newY + "px,0px)"
      });
    });
  };
  
  var stopParallax = function() {
    $('.parallax [class*="wave"]').css({
      transform: "translate(0px,0px)",
      transition: "all .7s"
    });
    setTimeout(function() {
      $('.parallax [class*="wave"]').css("transition", "");
    }, 700);
  };
  
  $(function() {
    $(".not-found").on("mousemove", parallax);
    $(".not-found").on("mouseleave", stopParallax);
  });
  
  // Add event listener to wave-island link
  document.addEventListener("DOMContentLoaded", function() {
    document.querySelector('a.wave-island').addEventListener('click', function(e) {
      e.preventDefault();
      document.querySelector('body').classList.add('fade-out');
      setTimeout(function() {
        window.location.href = "welcome.html";
      }, 500);
    });
  });
  