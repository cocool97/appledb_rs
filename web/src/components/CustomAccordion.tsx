import React from "react";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Typography,
} from "@mui/material";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";

interface AccordionMember {
  title: string;
  summary: string;
  component: React.ReactNode;
}

interface CustomAccordionProps {
  members: AccordionMember[];
}

export const CustomAccordion: React.FC<CustomAccordionProps> = ({
  members,
}) => {
  return (
    <>
      {members.map((element, index) => (
        <Accordion
          key={index}
          sx={{ backgroundColor: "transparent" }}
          slotProps={{ transition: { unmountOnExit: true } }}
        >
          <AccordionSummary
            expandIcon={<ExpandMoreIcon />}
            aria-controls={`panel${index}-content`}
            id={`panel${index}-header`}
          >
            <Typography component="span" sx={{ width: "33%", flexShrink: 0 }}>
              {element.title}
            </Typography>
            <Typography component="span">{element.summary}</Typography>
          </AccordionSummary>
          <AccordionDetails>{element.component}</AccordionDetails>
        </Accordion>
      ))}
    </>
  );
};
