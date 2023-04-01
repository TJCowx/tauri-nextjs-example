import { styled } from '@mui/material';
import { FC, PropsWithChildren } from 'react';
import NavDrawer from './NavDrawer';

const ContentContainer = styled('div')(() => ({
  marginLeft: '48px',
}));

const Layout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <div>
      <NavDrawer />
      <ContentContainer className="content-container">
        {children}
      </ContentContainer>
    </div>
  );
};

export default Layout;
