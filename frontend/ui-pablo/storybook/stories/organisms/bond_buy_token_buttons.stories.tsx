import { BuyButtons } from "@ui-pablo/app/components/Organisms";
import { Box } from "@mui/material";
import { ComponentStory } from "@storybook/react";
import BigNumber from "bignumber.js";

const BuyButtonsStories = () => {
  const bond = {
    tokenId1: "ksm",
    tokenId2: "pica",
    roi: 26,
    vesting_term: 7,
    tvl: new BigNumber(1500000),
    volumne: new BigNumber(132500000),
    discount_price: new BigNumber(2.3),
    market_price: new BigNumber(2.9),
    balance: new BigNumber(435),
    rewardable_amount: new BigNumber(0),
    buyable_amount: new BigNumber(500),
    pending_amount: new BigNumber(0),
    claimable_amount: new BigNumber(0),
    remaining_term: 7,
    vested_term: 0,
  } as const;
  return (
    <Box>
      <BuyButtons bond={bond} />
    </Box>
  );
};
export default {
  title: "organisms/Bond/BuyButtons",
  component: BuyButtons,
};

const Template: ComponentStory<typeof BuyButtons> = (args) => (
  <BuyButtonsStories {...args} />
);

export const Default = Template.bind({});
Default.args = {};