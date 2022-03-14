import React from 'react';
import { Avatar, List, Upload, Button } from 'antd';
import {
  FileZipTwoTone,
  FileMarkdownTwoTone,
  FolderOpenTwoTone,
  FilePdfTwoTone,
  FileUnknownTwoTone,
  FileTextTwoTone,
  UploadOutlined,
} from '@ant-design/icons';
import styled from '@emotion/styled';
import qs from 'qs';
import { blob } from 'stream/consumers';

interface PathList {
  list: {
    name: string;
    path_uri: string;
    ext: string;
    is_file: boolean;
    last_modified: string;
  }[];
}

const apiUrl = process.env.REACT_APP_API_URL;

const get_icon = (is_file: boolean, ext: string) => {
  if (is_file) {
    if (ext.toLowerCase() === 'md') {
      return <FileMarkdownTwoTone />;
    } else if (ext.toLowerCase() === 'pdf') {
      return <FilePdfTwoTone />;
    } else if (ext.toLowerCase() === 'zip') {
      return <FileZipTwoTone />;
    } else if (ext.toLowerCase() === 'txt') {
      return <FileTextTwoTone />;
    } else {
      return <FileUnknownTwoTone />;
    }
  }

  return <FolderOpenTwoTone />;
};

// <Avatar style={{ backgroundColor: '#87d068' }} icon={<UserOutlined />} />
export const PathList = ({ list }: PathList) => {
  return (
    <Container>
      <Header between={true}>
        <HeaderLeft>
          <h3> file path</h3>
        </HeaderLeft>
        <HeaderRight>
          <Upload>
            <Button icon={<UploadOutlined />}>Click to Upload</Button>
          </Upload>
        </HeaderRight>
      </Header>
      <List
        bordered={true}
        itemLayout="horizontal"
        dataSource={list}
        renderItem={(item) => {
          return (
            <List.Item>
              <List.Item.Meta
                avatar={<Avatar icon={get_icon(item.is_file, item.ext)} />}
                title={
                  <Button
                    type="link"
                    onClick={() => {
                      window
                        .fetch(`${apiUrl}/file?${qs.stringify(item)}`)
                        .then(async (response) => {
                          response.blob().then((blob) => {
                            let url = window.URL.createObjectURL(blob);
                            let a = document.createElement('a');
                            a.href = url;
                            a.download = item.name;
                            a.click();
                          });
                        });
                    }}
                  >
                    {item.name}
                  </Button>
                }
                description={item.last_modified}
              />
            </List.Item>
          );
        }}
      />
    </Container>
  );
};

const Row = styled.div<{
  gap?: number | boolean;
  between?: boolean;
  marginBottom?: number;
}>`
  display: flex;
  align-items: center;
  justify-content: ${(props) => (props.between ? 'space-between' : undefined)};
  margin_bottom: ${(props) => props.marginBottom + 'rem'};
  > * {
    margin-top: 0 !important;
    margin-bottom: 0 !important;
    margin-right: ${(props) =>
      typeof props.gap === 'number'
        ? props.gap + 'rem'
        : props.gap
        ? '2rem'
        : undefined};
  }
`;

const Header = styled(Row)`
  padding: 3.2rem;
  box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.1);
  z-index: 1;
`;

const HeaderLeft = styled.div``;
const HeaderRight = styled.div``;

const Container = styled.div`
  padding: 3.2rem;
`;
