<template>
  <div>
    <h1>{{job_description}}</h1>
    <span>{{organisation.name}}</span>
    <div v-if="hasContactInfo">
      {{contact.name}}
      <div v-if="email && email.length">
        E-Mail:
        {{email}}
        <span v-if="emailnote && emailnote.length">({{emailnote}})</span>
      </div>
      <div v-if="phone && phone.length">
        Phone:
        {{phone}}
        <span v-if="phonenote && phonenote.length">({{phonenote}})</span>
      </div>
    </div>
    <div v-if="disciplines && disciplines.length">Sportart: {{disciplines}}</div>
  </div>
</template>

<script>
export default {
  props: {
    job_description: String,
    organisation: Object,
    contact: Object,
    labels: Array
  },
  data() {
    return {
      disciplineKey: "Sportart"
    };
  },
  computed: {
    hasContactInfo() {
      return this.contact.options;
    },
    disciplines() {
      const disciplines = this.getLabelValue(this.disciplineKey);

      if (disciplines[0].values) {
        return disciplines[0].values.join(",");
      }

      return "";
    },
    email() {
      if (this.getContactValue("email")) {
        return this.getContactValue("email").value;
      }
      return "";
    },
    phone() {
      if (this.getContactValue("phone")) {
        return this.getContactValue("phone").value;
      }
      return "";
    },
    emailnote() {
      if (this.getContactValue("email")) {
        return this.getContactValue("email").note;
      }
      return "";
    },
    phonenote() {
      if (this.getContactValue("phone")) {
        return this.getContactValue("phone").note;
      }
      return "";
    }
  },
  methods: {
    getLabelValue(key) {
      return this.labels.filter(label => {
        return label.key === key;
      });
    },
    getContactValue(key) {
      if (!this.contact.options) {
        return "";
      }
      const option = this.contact.options.filter(option => {
        if (option[key]) {
          return true;
        }
        return false;
      });

      if (!option[0][key]) {
        return {};
      }
      return option[0][key];
    }
  }
};
</script>


