import { styled } from '@mui/material';
import clsx from 'clsx';
import { FC, PropsWithChildren } from 'react';
import NavDrawer from './NavDrawer';

const ContentContainer = styled('main')(() => ({
  marginLeft: '48px',
  overflow: 'auto',
  '&:not(.no-padding)': {
    padding: '24px 16px 48px',
  },
  '&::-webkit-scrollbar': {
    width: '16px',
  },
  '&::-webkit-scrollbar-thumb': {
    backgroundColor: '#ccc',
    borderRadius: '12px',
    border: '4px solid transparent',
    backgroundClip: 'content-box',
    minWidth: '16px',
    minHeigh: '16px',
  },
  '&::-webkit-scrollbar-track': {
    backgroundColor: 'transparent',
  },
}));

type Props = {
  disablePadding?: boolean;
};

const Layout: FC<PropsWithChildren<Props>> = ({
  children,
  disablePadding = false,
}) => (
  <div>
    <NavDrawer />
    <ContentContainer
      className={clsx({
        'content-container': true,
        'no-padding': disablePadding,
      })}
    >
      {children}
    </ContentContainer>
  </div>
);

Layout.defaultProps = {
  disablePadding: false,
};

export default Layout;
