import { List, ListItemButton } from '@mui/material';
import ListItemText from 'components/List/ListItemText';
import NextLink from 'next/link';
import { FC } from 'react';

const Home: FC = () => {
  return (
    <>
      <List>
        <NextLink className="reset" href="/initiative">
          <ListItemButton>
            <ListItemText
              primary="Initiative Tracker"
              secondary="Start tracking initiative for an encounter."
            />
          </ListItemButton>
        </NextLink>
        <NextLink className="reset" href="/creatures">
          <ListItemButton>
            <ListItemText
              primary="Creatures List"
              secondary="View, create, and edit creatures."
            />
          </ListItemButton>
        </NextLink>
        <NextLink className="reset" href="/">
          <ListItemButton>
            <ListItemText
              primary="Change Connection"
              secondary="Go back and change your connection string."
            />
          </ListItemButton>
        </NextLink>
      </List>
    </>
  );
};

export default Home;
