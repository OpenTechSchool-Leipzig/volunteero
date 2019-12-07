<template>
  <div>
    <Bot />
    <Modal v-show="showModal" @closeModal="showModal = false" />
  </div>
</template>

<script>
import Bot from "@/components/Bot";
import Modal from "@/components/Modal";

export default {
  components: { Bot, Modal },
  data() {
    return {
      showModal: false
    };
  }
};
</script>
