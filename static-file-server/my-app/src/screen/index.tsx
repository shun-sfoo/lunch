import React from 'react';
import { Avatar, List } from 'antd';

interface PathList {
  list: {
    name: string;
    path_uri: string;
    ext: string;
    is_file: boolean;
    last_modified: string;
  }[];
}

export const PathList = ({ list }: PathList) => {
  return (
    <List
      itemLayout="horizontal"
      dataSource={list}
      renderItem={(item) => {
        return (
          <List.Item>
            <List.Item.Meta
              avatar={<Avatar src="https://joeschmoe.io/api/v1/random" />}
              title={<a href="https://ant.design">{item.name}</a>}
              description="Ant Design, a design language for background applications, is refined by Ant UED Team"
            />
          </List.Item>
        );
      }}
    />
  );
};
