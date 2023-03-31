import { Button, Paper, styled } from '@mui/material';
import { invoke } from '@tauri-apps/api/tauri';
import RHFTextField from 'components/Fields/RHF/RHFTextField';
import { useState } from 'react';
import { useForm } from 'react-hook-form';

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

const StyledForm = styled('form')(() => ({
  height: '100%',
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'space-between',
  '& button.connect': { alignSelf: 'flex-end' },
}));

type FormState = {
  connectionString: string;
};

export default function Connect() {
  const { control, handleSubmit } = useForm<FormState>({
    defaultValues: {
      connectionString: '',
    },
  });

  const onSubmit = (data: FormState) => {
    invoke('connect_db', { conn_str: data.connectionString })
      .then(() => console.log('TODO: Redirect'))
      .catch((e) => {
        console.error(e);
      });
  };

  return (
    <Root style={{ paddingTop: '16px' }}>
      <StyledPaper>
        <StyledForm onSubmit={handleSubmit(onSubmit)}>
          <RHFTextField
            fieldName="connectionString"
            label="Connection"
            control={control}
            isRequired
          />
          <Button className="connect" type="submit" variant="contained">
            Connect
          </Button>
        </StyledForm>
      </StyledPaper>
    </Root>
  );
}
