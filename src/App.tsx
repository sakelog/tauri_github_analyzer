import { useState, useEffect } from 'react';

// lib
import { traffic } from 'lib/github/getContent';

// component
import {
  Grid,
  Typography,
  List,
  ListItem,
  ListItemText,
} from '@mui/material';

const App = () => {
  const [trafficResults, setTrafficResults] = useState<
    Array<GitHub.TrafficResult>
  >([]);

  useEffect(() => {
    const onGetTraffics = async () => {
      const results = await traffic.getTraffic();
      setTrafficResults(results);
    };
    onGetTraffics();
  }, []);

  return (
    <Grid container spacing={2} alignItems="stretch">
      {trafficResults.map((item) => (
        <Grid key={item.name} item xs={4} overflow="hidden">
          <Typography
            variant="h5"
            component="h2"
            display="block"
          >
            {item.name}
          </Typography>
          <span>{item.count}</span>
          <List>
            {item.views.map((view) => (
              <ListItem
                key={item.name + '_' + view.timestamp}
              >
                <ListItemText>
                  {view.timestamp}
                </ListItemText>
                <ListItemText>{view.count}</ListItemText>
              </ListItem>
            ))}
          </List>
        </Grid>
      ))}
    </Grid>
  );
};

export default App;
