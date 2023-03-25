import { Paper, styled } from '@mui/material';
import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

const Root = styled('div')(() => ({
  height: '100%',
  width: '100%',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
}));

const StyledPaper = styled(Paper)(() => ({
  width: '400px',
  height: '200px',
  padding: '24px',
}));

export default function Connect() {
  return (
    <Root style={{ paddingTop: '16px' }}>
      <StyledPaper>Hello World</StyledPaper>
    </Root>
  );
}
