<div class="stars__wrapper">
  <div class="stars" />
  <div class="stars2" />
  <div class="stars3" />
</div>
<div class="stars__content">
  <slot />
</div>

<style lang="scss">
  @function multiple-box-shadow($n) {
    $value: "#{random(100)}vh #{random(100)}vw #FFF";
    @for $i from 2 through $n {
      $value: "#{$value} , #{random(100)}vw #{random(100)}vw #FFF";
    }

    @return unquote($value);
  }

  $shadows-small: multiple-box-shadow(700);
  $shadows-medium: multiple-box-shadow(200);
  $shadows-big: multiple-box-shadow(100);

  .stars {
    width: 1px;
    height: 1px;
    background: transparent;
    box-shadow: $shadows-small;
    animation: animStar 50s linear infinite;
  }

  .stars:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 1px;
    height: 1px;
    background: transparent;
    box-shadow: $shadows-small;
  }
  .stars2 {
    width: 2px;
    height: 2px;
    background: transparent;
    box-shadow: $shadows-medium;
    animation: animStar 100s linear infinite;
  }
  .stars2:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 2px;
    height: 2px;
    background: transparent;
    box-shadow: $shadows-medium;
  }
  .stars3 {
    width: 3px;
    height: 3px;
    background: transparent;
    box-shadow: $shadows-big;
    animation: animStar 150s linear infinite;
  }
  .stars3:after {
    content: " ";
    position: absolute;
    top: 2000px;
    width: 3px;
    height: 3px;
    background: transparent;
    box-shadow: $shadows-big;
  }
  .stars__wrapper {
    position: fixed;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
  }
  .stars__content {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 10;
    overflow-x: hidden;
  }
  @keyframes animStar {
    to {
      transform: translateY(0);
    }
    from {
      transform: translateY(-100vh);
    }
  }
</style>
