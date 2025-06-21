import { createListCollection } from "@chakra-ui/react";

export const subscriptions = createListCollection({
  items: [
    {
      name: "Vitalik Feet Pics Monthly",
      id: "vfp_monthly",
      avatar: "vfg.svg",
    },

    {
      name: "zketflix Monthly {coming soon}",
      id: "zketflix_Monthly",
      avatar: "zketflix.svg",
    },
  ],
  itemToString: (item) => item.name,
  itemToValue: (item) => item.id,
});

export const months = createListCollection({
  items: [
    { name: "January", id: "01" },
    { name: "February", id: "02" },
    { name: "March", id: "03" },
    { name: "April", id: "04" },
    { name: "May", id: "05" },
    { name: "June", id: "06" },
    { name: "July", id: "07" },
    { name: "August", id: "08" },
    { name: "September", id: "09" },
    { name: "October", id: "10" },
    { name: "November", id: "11" },
    { name: "December", id: "12" },
  ],
  itemToString: (item) => item.name,
  itemToValue: (item) => item.id,
});

export const years = createListCollection({
  items: [
    { name: "2025", id: "2025" },
    { name: "2026", id: "2026" },
  ],
  itemToString: (item) => item.name,
  itemToValue: (item) => item.id,
});
