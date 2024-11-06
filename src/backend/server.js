const express = require('express');
const explorerRoutes = require('./routes/explorer');
const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());
app.use('/api/explorer', explorerRoutes);

app.listen(port, () => {
  console.log(`Server running on port ${port}`);
});
