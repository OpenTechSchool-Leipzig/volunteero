<template>
  <div>
    <Result v-for="result in results" :key="result.organisation.id" v-bind="result" />
  </div>
</template>

<script>
import Result from "@/components/Result";

export default {
  components: {
    Result
  },
  data() {
    return {
      results: [
        {
          title: "Trainer",
          organisation: {
            id: "42",
            name: "Fußball Verein Leipzig e.V."
          },
          locations: [
            {
              city: "Leipzig",
              postcode: "04229",
              house_number: "37 F",
              street: "Musterallee",
              name: "Kaninchenrennbahn"
            }
          ],
          contact: {
            name: "Herr Müller",
            options: [
              {
                email: {
                  value: "mueller@fussball-leipzig.de",
                  note: null
                }
              },
              {
                phone: {
                  value: "+4927182818284",
                  note: "Nur bis 18:00Uhr"
                }
              }
            ]
          },
          labels: [
            {
              key: "Sportart",
              values: ["Fußball"]
            },
            {
              key: "Aufgabenfeld",
              values: ["Vereinsleben", "Trainigsbetrieb"]
            }
          ]
        }
      ]
    };
  }
};
</script>

<style>
</style>

