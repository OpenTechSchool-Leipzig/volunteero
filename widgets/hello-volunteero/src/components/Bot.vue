<template>
  <div v-if="show" class="bot" @click.prevent="openModal">
        <button
          @click.stop.prevent="show = !show"
          class="close-button"
          aria-label="close"
        ><bold>✘</bold></button>
    <div class="speech-bubble">Lust auf Ehrenamt?</div>
    <LamaSvg />
  </div>
</template>

<script>
import LamaSvg from "@/svg/Lama";

export default {
  components: { LamaSvg },
  methods: {
    openModal() {
      this.$emit("showModal");
    }
  }, data() {
    return {
      show: true,
    };
  },
};
</script>

<style>
.bot {
  width: 150px;
  cursor: pointer;
}
.close-button {
    position: fixed;
    right: 0px;
    bottom: 230px;
    z-index: 9999;
    color: white;
    background-color:transparent;
    border: 0px
}
button.close-button:hover {
  background:transparent;
  color: rgba(255, 0, 0, 0.609);
  cursor:pointer;
}
.speech-bubble {
  position: relative;
  color: white;
  font-size: 1.2em;
  line-height: 25px;
  text-align: center;
  padding: 20px;
  margin-bottom: 5px;
  border-radius: 0.4em;
  width: 100%;
  background-color: #009ee0;
  background-color: var(--primary-color);
}
.speech-bubble:after {
  content: "";
  position: absolute;
  bottom: 0;
  left: 50%;
  width: 0;
  height: 0;
  border: 0.8em solid transparent;
  border-top-color: #009ee0;
  border-top-color: var(--primary-color);
  border-bottom: 0;
  border-left: 0;
  margin-left: -0.312em;
  margin-bottom: -0.8em;
}
.fixed-right {
  position: fixed;
  right: 10px;
  bottom: -30px;
}
.fixed-left {
  position: fixed;
  left: 10px;
  bottom: -30px;
}
</style>
