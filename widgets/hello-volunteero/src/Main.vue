<template>
  <div class="volunteero">
    <Bot @showModal="showModal = true" :class="botPosition" :style="botStyle" />
    <Modal v-show="showModal" @closeModal="showModal = false" />
  </div>
</template>

<script>
import Bot from "@/components/Bot";
import Modal from "@/components/Modal";

export default {
  components: { Bot, Modal },
  props: ["botPosition", "botSize"],
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

<style>
.volunteero * {
  box-sizing: border-box;
}
</style>
