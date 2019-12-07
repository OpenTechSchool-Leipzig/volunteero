<template>
  <div class="volunteero" :class="position">
    <Bot @showModal="showModal = true" />
    <Modal v-show="showModal" @closeModal="showModal = false" />
  </div>
</template>

<script>
import Bot from "@/components/Bot";
import Modal from "@/components/Modal";

export default {
  components: { Bot, Modal },
  props: ["position"],
  data() {
    return {
      showModal: false
    };
  }
};
</script>

<style>
.volunteero * {
  box-sizing: border-box;
}
.fixed-right {
  position: absolute;
  position: fixed;
  right: 10px;
  bottom: -30px;
}
.fixed-left {
  position: absolute;
  position: fixed;
  left: 10px;
  bottom: -30px;
}
</style>
