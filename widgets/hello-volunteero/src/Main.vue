<template>
  <div class="volunteero">
    <Style>.volunteero{ --primary-color: {{ primaryColor }} }</Style>
    <Bot @showModal="showModal = true" :class="botPosition" :style="botStyle" />
    <Modal v-show="showModal" @closeModal="showModal = false" />
  </div>
</template>

<script>
import Bot from "@/components/Bot";
import Modal from "@/components/Modal";
import Style from "@/components/Style";

export default {
  components: { Bot, Modal, Style },
  props: {
    botPosition: {
      type: String,
      default: "fixed-right"
    },
    botSize: {
      type: Number,
      default: 1
    },
    primaryColor: {
      type: String,
      default: "#009ee0"
    }
  },
  data() {
    return {
      showModal: false
    };
  },
  computed: {
    botStyle() {
      return `transform:scale(${this.botSize});`;
    }
  }
};
</script>

<style lang="scss">
.volunteero * {
  font-family: Arial, Helvetica, sans-serif;
  box-sizing: border-box;
}

.volunteero {
  button {
    outline: none;
  }
}
</style>
